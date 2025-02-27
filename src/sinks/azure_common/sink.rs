use crate::sinks::util::partitioner::KeyPartitioner;
use crate::{
    config::SinkContext,
    event::Event,
    sinks::util::{RequestBuilder, SinkBuilderExt},
};
use async_trait::async_trait;
use futures::stream::BoxStream;
use futures_util::StreamExt;
use std::{fmt, num::NonZeroUsize};
use tower::Service;
use vector_core::buffers::Acker;
use vector_core::stream::{BatcherSettings, DriverResponse};
use vector_core::{buffers::Ackable, event::Finalizable, sink::StreamSink};

pub struct AzureBlobSink<Svc, RB> {
    acker: Acker,
    service: Svc,
    request_builder: RB,
    partitioner: KeyPartitioner,
    batcher_settings: BatcherSettings,
}

impl<Svc, RB> AzureBlobSink<Svc, RB> {
    pub fn new(
        cx: SinkContext,
        service: Svc,
        request_builder: RB,
        partitioner: KeyPartitioner,
        batcher_settings: BatcherSettings,
    ) -> Self {
        Self {
            acker: cx.acker(),
            service,
            request_builder,
            partitioner,
            batcher_settings,
        }
    }
}

impl<Svc, RB> AzureBlobSink<Svc, RB>
where
    Svc: Service<RB::Request> + Send + 'static,
    Svc::Future: Send + 'static,
    Svc::Response: DriverResponse + Send + 'static,
    Svc::Error: fmt::Debug + Into<crate::Error> + Send,
    RB: RequestBuilder<(String, Vec<Event>)> + Send + Sync + 'static,
    RB::Error: fmt::Debug + Send,
    RB::Request: Ackable + Finalizable + Send,
{
    async fn run_inner(self: Box<Self>, input: BoxStream<'_, Event>) -> Result<(), ()> {
        let partitioner = self.partitioner;
        let settings = self.batcher_settings;

        let builder_limit = NonZeroUsize::new(64);
        let request_builder = self.request_builder;

        let sink = input
            .batched(partitioner, settings)
            .filter_map(|(key, batch)| async move { key.map(move |k| (k, batch)) })
            .request_builder(builder_limit, request_builder)
            .filter_map(|request| async move {
                match request {
                    Err(e) => {
                        error!("Failed to build Azure Blob request: {:?}.", e);
                        None
                    }
                    Ok(req) => Some(req),
                }
            })
            .into_driver(self.service, self.acker);

        sink.run().await
    }
}

#[async_trait]
impl<Svc, RB> StreamSink for AzureBlobSink<Svc, RB>
where
    Svc: Service<RB::Request> + Send + 'static,
    Svc::Future: Send + 'static,
    Svc::Response: DriverResponse + Send + 'static,
    Svc::Error: fmt::Debug + Into<crate::Error> + Send,
    RB: RequestBuilder<(String, Vec<Event>)> + Send + Sync + 'static,
    RB::Error: fmt::Debug + Send,
    RB::Request: Ackable + Finalizable + Send,
{
    async fn run(mut self: Box<Self>, input: BoxStream<'_, Event>) -> Result<(), ()> {
        self.run_inner(input).await
    }
}
