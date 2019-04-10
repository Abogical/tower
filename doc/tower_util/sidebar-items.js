initSidebarItems({"enum":[["Either","Combine two different service types into a single type."]],"fn":[["service_fn","Returns a new `ServiceFn` with the given closure."]],"mod":[["error","Error types"],["future","Future types"],["layer",""]],"struct":[["BoxService","A boxed `Service + Send` trait object."],["CallAll","This is a `futures::Stream` of responses resulting from calling the wrapped `tower::Service` for each request received on the wrapped `Stream`."],["CallAllUnordered","A stream of responses received from the inner service in received order."],["Oneshot","A `Future` consuming a `Service` and request, waiting until the `Service` is ready, and then calling `Service::call` with the request, and waiting for that `Future`."],["Optional","Optionally forwards requests to an inner service."],["Ready","Future yielding a `Service` once the service is ready to process a request"],["ServiceFn","A `Service` implemented by a closure."],["UnsyncBoxService","A boxed `Service` trait object."]],"trait":[["MakeConnection","The MakeConnection trait is used to create transports"],["MakeService","Creates new `Service` values."]]});