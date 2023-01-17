use fastly::ObjectStore;
use fastly::{Error, Request, Response};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Log out which version of the Fastly Service is responding to this request.
    // This is useful to know when debugging.
    if let Ok(fastly_service_version) = std::env::var("FASTLY_SERVICE_VERSION") {
        println!("FASTLY_SERVICE_VERSION: {}", fastly_service_version);
    }

    /*
        Construct an ObjectStore instance which is connected to the Object Store named `my-store`

        [Documentation for the ObjectStore open method can be found here](https://docs.rs/fastly/latest/fastly/struct.ObjectStore.html#method.open)
    */
    let mut store =
        ObjectStore::open("my-store").map(|store| store.expect("ObjectStore exists"))?;

    let path = req.get_path();
    if path == "/readme" {
        let entry = store.lookup("readme")?;

        return match entry {
            // Stream the value back to the user-agent.
            Some(entry) => Ok(Response::from_body(entry)),
            None => Ok(Response::from_body("Not Found").with_status(404)),
        };
    } else {
        /*
        Adds or updates the key `hello` in the Object Store with the value `world`.

        Note: Object stores are eventually consistent, this means that the updated value associated
        with the key may not be available to read from all edge locations immediately and some edge
        locations may continue returning the previous value associated with the key.

        [Documentation for the insert method can be found here](https://docs.rs/fastly/latest/fastly/struct.ObjectStore.html#method.insert)
        */
        store.insert("hello", "world")?;

        /*
        Retrieve the value associated with the key `hello` in the Object Store.
        If the key does not exist, then `None` is returned.
        If the key does exist, then an `Some<Body>` is returned.

        [Documentation for the lookup method can be found here](https://docs.rs/fastly/latest/fastly/struct.ObjectStore.html#method.lookup)
        */
        let entry = store.lookup("hello")?;

        return match entry {
            // Stream the value back to the user-agent.
            Some(entry) => Ok(Response::from_body(entry)),
            None => Ok(Response::from_body("Not Found").with_status(404)),
        };
    }
}
