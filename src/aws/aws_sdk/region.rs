use crate::aws::region::RegionOrEndpoint;
use aws_sdk_sqs::Endpoint;
use aws_types::region::Region;
use http::Uri;
use std::str::FromStr;

impl RegionOrEndpoint {
    pub fn endpoint(&self) -> crate::Result<Option<Endpoint>> {
        if let Some(endpoint) = &self.endpoint {
            Ok(Some(Endpoint::immutable(Uri::from_str(endpoint)?)))
        } else {
            Ok(None)
        }
    }

    pub fn region(&self) -> Option<Region> {
        self.region.clone().map(Region::new)
    }
}
