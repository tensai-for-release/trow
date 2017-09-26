use rocket;
use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::request::Request;

/// Exporting all routes for the project
pub mod empty;
pub mod uuid;

/// Encapsulate a response from the registry
/// Currently the full type definition is not possible (26 Sept 2017),
/// The hope is that we can restrict the following to RegistryTrait in
/// the future. 
///
/// We simply pass in a Struct/Enum into the type-constructor, and as
/// long as we have implemented the RegistryTrait trait, then
/// everything just works.
pub type MaybeResponse<A/*: RegistryTrait*/> = RegistryResponse<A>;

/// Two constructors to ease sending a success/fail response.
/// One simply calls MaybeResponse::ok(A: RegistryTrait).
impl<A: RegistryTrait> MaybeResponse<A>  {
    pub fn ok(val: A) -> Self
    where A: RegistryTrait{
        RegistryResponse(Ok(val))
    }

    pub fn err(val: A) -> Self
    where A: RegistryTrait {
        RegistryResponse(Err(val))
    }
}

/// This trait is used to enforce certain conventions when building a response.
/// There are two functions, ok() which is called when an Ok result is indicated.
/// err() which is called when an Err result is indicated.
pub trait RegistryTrait {
    fn ok<'r>(self, req: &Request) -> Result<Response<'r>, Status>;
    fn err<'r>(self, req: &Request) -> Result<Response<'r>, Status>;
}

/// Performs runtime dispatch to return call success or failure.
#[derive(Debug)]
pub struct RegistryResponse<R: RegistryTrait>(pub Result<R, R>);

impl<'r, R> Responder<'r> for RegistryResponse<R>
    where R: Responder<'r> + RegistryTrait {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        debug!("In the Registry Response");

        let req = match self.0 {
            Ok(r) => r.ok(req)?,
            Err(e) => e.err(req)?,
        };
        let header = rocket::http::Header::new("Docker-Distribution-API-Version", "registry/2.0");

        Response::build()
            .header(header)
            .merge(req)
            .ok()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }

    #[test]
    fn panicify() {
        panic!("something happened");
    }
}
