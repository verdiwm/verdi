#![allow(unused)]
#![allow(async_fn_in_trait)]
pub mod wayland {
    pub mod wl_display {
        enum r#Error {}
        #[doc = r#"The core global object.  This is a special singleton object.  It"#]
        #[doc = r#"is used for internal Wayland protocol features."#]
        pub trait r#WlDisplay {
            const INTERFACE: &'static str = "wl_display";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_display -> sync");
                        Self::r#sync(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1 => {
                        tracing::debug!("wl_display -> get_registry");
                        Self::r#get_registry(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"The sync request asks the server to emit the 'done' event"#]
            #[doc = r#"on the returned wl_callback object.  Since requests are"#]
            #[doc = r#"handled in-order and events are delivered in-order, this can"#]
            #[doc = r#"be used as a barrier to ensure all previous requests and the"#]
            #[doc = r#"resulting events have been handled."#]
            #[doc = r#""#]
            #[doc = r#"The object returned by this request will be destroyed by the"#]
            #[doc = r#"compositor after the callback is fired and as such the client must not"#]
            #[doc = r#"attempt to use it after that point."#]
            #[doc = r#""#]
            #[doc = r#"The callback_data passed in the callback is undefined and should be ignored."#]
            async fn r#sync(
                client: &mut crate::Client,
                r#callback: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"This request creates a registry object that allows the client"#]
            #[doc = r#"to list and bind the global objects available from the"#]
            #[doc = r#"compositor."#]
            #[doc = r#""#]
            #[doc = r#"It should be noted that the server side resources consumed in"#]
            #[doc = r#"response to a get_registry request can only be released when the"#]
            #[doc = r#"client disconnects, not when the client side proxy is destroyed."#]
            #[doc = r#"Therefore, clients should invoke get_registry as infrequently as"#]
            #[doc = r#"possible to avoid wasting memory."#]
            async fn r#get_registry(
                client: &mut crate::Client,
                r#registry: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"The error event is sent out when a fatal (non-recoverable)"#]
            #[doc = r#"error has occurred.  The object_id argument is the object"#]
            #[doc = r#"where the error occurred, most often in response to a request"#]
            #[doc = r#"to that object.  The code identifies the error and is defined"#]
            #[doc = r#"by the object interface.  As such, each interface defines its"#]
            #[doc = r#"own set of error codes.  The message is a brief description"#]
            #[doc = r#"of the error, for (debugging) convenience."#]
            async fn r#error(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#object_id: crate::wire::ObjectId,
                r#code: u32,
                r#message: String,
            ) -> crate::Result<()> {
                tracing::debug!("wl_display -> error");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_object(Some(object_id))
                    .put_uint(code)
                    .put_string(Some(message))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event is used internally by the object ID management"#]
            #[doc = r#"logic. When a client deletes an object that it had created,"#]
            #[doc = r#"the server will send this event to acknowledge that it has"#]
            #[doc = r#"seen the delete request. When the client receives this event,"#]
            #[doc = r#"it will know that it can safely reuse the object ID."#]
            async fn r#delete_id(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#id: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_display -> delete_id");
                let payload = crate::wire::PayloadBuilder::new().put_uint(id).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod wl_registry {
        #[doc = r#"The singleton global registry object.  The server has a number of"#]
        #[doc = r#"global objects that are available to all clients.  These objects"#]
        #[doc = r#"typically represent an actual object in the server (for example,"#]
        #[doc = r#"an input device) or they are singleton objects that provide"#]
        #[doc = r#"extension functionality."#]
        #[doc = r#""#]
        #[doc = r#"When a client creates a registry object, the registry object"#]
        #[doc = r#"will emit a global event for each global currently in the"#]
        #[doc = r#"registry.  Globals come and go as a result of device or"#]
        #[doc = r#"monitor hotplugs, reconfiguration or other events, and the"#]
        #[doc = r#"registry will send out global and global_remove events to"#]
        #[doc = r#"keep the client up to date with the changes.  To mark the end"#]
        #[doc = r#"of the initial burst of events, the client can use the"#]
        #[doc = r#"wl_display.sync request immediately after calling"#]
        #[doc = r#"wl_display.get_registry."#]
        #[doc = r#""#]
        #[doc = r#"A client can bind to a global object by using the bind"#]
        #[doc = r#"request.  This creates a client-side handle that lets the object"#]
        #[doc = r#"emit events to the client and lets the client invoke requests on"#]
        #[doc = r#"the object."#]
        pub trait r#WlRegistry {
            const INTERFACE: &'static str = "wl_registry";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_registry -> bind");
                        Self::r#bind(client, message.uint()?, message.new_id()?).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Binds a new, client-created object to the server using the"#]
            #[doc = r#"specified name as the identifier."#]
            async fn r#bind(
                client: &mut crate::Client,
                r#name: u32,
                r#id: crate::wire::NewId,
            ) -> crate::Result<()>;
            #[doc = r#"Notify the client of global objects."#]
            #[doc = r#""#]
            #[doc = r#"The event notifies the client that a global object with"#]
            #[doc = r#"the given name is now available, and it implements the"#]
            #[doc = r#"given version of the given interface."#]
            async fn r#global(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#name: u32,
                r#interface: String,
                r#version: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_registry -> global");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(name)
                    .put_string(Some(interface))
                    .put_uint(version)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Notify the client of removed global objects."#]
            #[doc = r#""#]
            #[doc = r#"This event notifies the client that the global identified"#]
            #[doc = r#"by name is no longer available.  If the client bound to"#]
            #[doc = r#"the global using the bind request, the client should now"#]
            #[doc = r#"destroy that object."#]
            #[doc = r#""#]
            #[doc = r#"The object remains valid and requests to the object will be"#]
            #[doc = r#"ignored until the client destroys it, to avoid races between"#]
            #[doc = r#"the global going away and a client sending a request to it."#]
            async fn r#global_remove(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#name: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_registry -> global_remove");
                let payload = crate::wire::PayloadBuilder::new().put_uint(name).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod wl_callback {
        #[doc = r#"Clients can handle the 'done' event to get notified when"#]
        #[doc = r#"the related request is done."#]
        #[doc = r#""#]
        #[doc = r#"Note, because wl_callback objects are created from multiple independent"#]
        #[doc = r#"factory interfaces, the wl_callback interface is frozen at version 1."#]
        pub trait r#WlCallback {
            const INTERFACE: &'static str = "wl_callback";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Notify the client when the related request is done."#]
            async fn r#done(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#callback_data: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_callback -> done");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(callback_data)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod wl_compositor {
        #[doc = r#"A compositor.  This object is a singleton global.  The"#]
        #[doc = r#"compositor is in charge of combining the contents of multiple"#]
        #[doc = r#"surfaces into one displayable output."#]
        pub trait r#WlCompositor {
            const INTERFACE: &'static str = "wl_compositor";
            const VERSION: u32 = 6;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_compositor -> create_surface");
                        Self::r#create_surface(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1 => {
                        tracing::debug!("wl_compositor -> create_region");
                        Self::r#create_region(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Ask the compositor to create a new surface."#]
            async fn r#create_surface(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"Ask the compositor to create a new region."#]
            async fn r#create_region(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
            ) -> crate::Result<()>;
        }
    }
    pub mod wl_shm_pool {
        #[doc = r#"The wl_shm_pool object encapsulates a piece of memory shared"#]
        #[doc = r#"between the compositor and client.  Through the wl_shm_pool"#]
        #[doc = r#"object, the client can allocate shared memory wl_buffer objects."#]
        #[doc = r#"All objects created through the same pool share the same"#]
        #[doc = r#"underlying mapped memory. Reusing the mapped memory avoids the"#]
        #[doc = r#"setup/teardown overhead and is useful when interactively resizing"#]
        #[doc = r#"a surface or for many small buffers."#]
        pub trait r#WlShmPool {
            const INTERFACE: &'static str = "wl_shm_pool";
            const VERSION: u32 = 2;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_shm_pool -> create_buffer");
                        Self::r#create_buffer(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.uint()?,
                        )
                        .await
                    }
                    1 => {
                        tracing::debug!("wl_shm_pool -> destroy");
                        Self::r#destroy(client).await
                    }
                    2 => {
                        tracing::debug!("wl_shm_pool -> resize");
                        Self::r#resize(client, message.int()?).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Create a wl_buffer object from the pool."#]
            #[doc = r#""#]
            #[doc = r#"The buffer is created offset bytes into the pool and has"#]
            #[doc = r#"width and height as specified.  The stride argument specifies"#]
            #[doc = r#"the number of bytes from the beginning of one row to the beginning"#]
            #[doc = r#"of the next.  The format is the pixel format of the buffer and"#]
            #[doc = r#"must be one of those advertised through the wl_shm.format event."#]
            #[doc = r#""#]
            #[doc = r#"A buffer will keep a reference to the pool it was created from"#]
            #[doc = r#"so it is valid to destroy the pool immediately after creating"#]
            #[doc = r#"a buffer from it."#]
            async fn r#create_buffer(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
                r#offset: i32,
                r#width: i32,
                r#height: i32,
                r#stride: i32,
                r#format: u32,
            ) -> crate::Result<()>;
            #[doc = r#"Destroy the shared memory pool."#]
            #[doc = r#""#]
            #[doc = r#"The mmapped memory will be released when all"#]
            #[doc = r#"buffers that have been created from this pool"#]
            #[doc = r#"are gone."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"This request will cause the server to remap the backing memory"#]
            #[doc = r#"for the pool from the file descriptor passed when the pool was"#]
            #[doc = r#"created, but using the new size.  This request can only be"#]
            #[doc = r#"used to make the pool bigger."#]
            #[doc = r#""#]
            #[doc = r#"This request only changes the amount of bytes that are mmapped"#]
            #[doc = r#"by the server and does not touch the file corresponding to the"#]
            #[doc = r#"file descriptor passed at creation time. It is the client's"#]
            #[doc = r#"responsibility to ensure that the file is at least as big as"#]
            #[doc = r#"the new pool size."#]
            async fn r#resize(client: &mut crate::Client, r#size: i32) -> crate::Result<()>;
        }
    }
    pub mod wl_shm {
        enum r#Error {}
        enum r#Format {}
        #[doc = r#"A singleton global object that provides support for shared"#]
        #[doc = r#"memory."#]
        #[doc = r#""#]
        #[doc = r#"Clients can create wl_shm_pool objects using the create_pool"#]
        #[doc = r#"request."#]
        #[doc = r#""#]
        #[doc = r#"On binding the wl_shm object one or more format events"#]
        #[doc = r#"are emitted to inform clients about the valid pixel formats"#]
        #[doc = r#"that can be used for buffers."#]
        pub trait r#WlShm {
            const INTERFACE: &'static str = "wl_shm";
            const VERSION: u32 = 2;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_shm -> create_pool");
                        Self::r#create_pool(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    1 => {
                        tracing::debug!("wl_shm -> release");
                        Self::r#release(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Create a new wl_shm_pool object."#]
            #[doc = r#""#]
            #[doc = r#"The pool can be used to create shared memory based buffer"#]
            #[doc = r#"objects.  The server will mmap size bytes of the passed file"#]
            #[doc = r#"descriptor, to use as backing memory for the pool."#]
            async fn r#create_pool(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
                r#fd: std::os::fd::RawFd,
                r#size: i32,
            ) -> crate::Result<()>;
            #[doc = r#"Using this request a client can tell the server that it is not going to"#]
            #[doc = r#"use the shm object anymore."#]
            #[doc = r#""#]
            #[doc = r#"Objects created via this interface remain unaffected."#]
            async fn r#release(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Informs the client about a valid pixel format that"#]
            #[doc = r#"can be used for buffers. Known formats include"#]
            #[doc = r#"argb8888 and xrgb8888."#]
            async fn r#format(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#format: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_shm -> format");
                let payload = crate::wire::PayloadBuilder::new().put_uint(format).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod wl_buffer {
        #[doc = r#"A buffer provides the content for a wl_surface. Buffers are"#]
        #[doc = r#"created through factory interfaces such as wl_shm, wp_linux_buffer_params"#]
        #[doc = r#"(from the linux-dmabuf protocol extension) or similar. It has a width and"#]
        #[doc = r#"a height and can be attached to a wl_surface, but the mechanism by which a"#]
        #[doc = r#"client provides and updates the contents is defined by the buffer factory"#]
        #[doc = r#"interface."#]
        #[doc = r#""#]
        #[doc = r#"Color channels are assumed to be electrical rather than optical (in other"#]
        #[doc = r#"words, encoded with a transfer function) unless otherwise specified. If"#]
        #[doc = r#"the buffer uses a format that has an alpha channel, the alpha channel is"#]
        #[doc = r#"assumed to be premultiplied into the electrical color channel values"#]
        #[doc = r#"(after transfer function encoding) unless otherwise specified."#]
        #[doc = r#""#]
        #[doc = r#"Note, because wl_buffer objects are created from multiple independent"#]
        #[doc = r#"factory interfaces, the wl_buffer interface is frozen at version 1."#]
        pub trait r#WlBuffer {
            const INTERFACE: &'static str = "wl_buffer";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_buffer -> destroy");
                        Self::r#destroy(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Destroy a buffer. If and how you need to release the backing"#]
            #[doc = r#"storage is defined by the buffer factory interface."#]
            #[doc = r#""#]
            #[doc = r#"For possible side-effects to a surface, see wl_surface.attach."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Sent when this wl_buffer is no longer used by the compositor."#]
            #[doc = r#"The client is now free to reuse or destroy this buffer and its"#]
            #[doc = r#"backing storage."#]
            #[doc = r#""#]
            #[doc = r#"If a client receives a release event before the frame callback"#]
            #[doc = r#"requested in the same wl_surface.commit that attaches this"#]
            #[doc = r#"wl_buffer to a surface, then the client is immediately free to"#]
            #[doc = r#"reuse the buffer and its backing storage, and does not need a"#]
            #[doc = r#"second buffer for the next surface content update. Typically"#]
            #[doc = r#"this is possible, when the compositor maintains a copy of the"#]
            #[doc = r#"wl_surface contents, e.g. as a GL texture. This is an important"#]
            #[doc = r#"optimization for GL(ES) compositors with wl_shm clients."#]
            async fn r#release(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("wl_buffer -> release");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod wl_data_offer {
        enum r#Error {}
        #[doc = r#"A wl_data_offer represents a piece of data offered for transfer"#]
        #[doc = r#"by another client (the source client).  It is used by the"#]
        #[doc = r#"copy-and-paste and drag-and-drop mechanisms.  The offer"#]
        #[doc = r#"describes the different mime types that the data can be"#]
        #[doc = r#"converted to and provides the mechanism for transferring the"#]
        #[doc = r#"data directly from the source client."#]
        pub trait r#WlDataOffer {
            const INTERFACE: &'static str = "wl_data_offer";
            const VERSION: u32 = 3;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_data_offer -> accept");
                        Self::r#accept(client, message.uint()?, message.string()?).await
                    }
                    1 => {
                        tracing::debug!("wl_data_offer -> receive");
                        Self::r#receive(
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                        )
                        .await
                    }
                    2 => {
                        tracing::debug!("wl_data_offer -> destroy");
                        Self::r#destroy(client).await
                    }
                    3 => {
                        tracing::debug!("wl_data_offer -> finish");
                        Self::r#finish(client).await
                    }
                    4 => {
                        tracing::debug!("wl_data_offer -> set_actions");
                        Self::r#set_actions(client, message.uint()?, message.uint()?).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Indicate that the client can accept the given mime type, or"#]
            #[doc = r#"NULL for not accepted."#]
            #[doc = r#""#]
            #[doc = r#"For objects of version 2 or older, this request is used by the"#]
            #[doc = r#"client to give feedback whether the client can receive the given"#]
            #[doc = r#"mime type, or NULL if none is accepted; the feedback does not"#]
            #[doc = r#"determine whether the drag-and-drop operation succeeds or not."#]
            #[doc = r#""#]
            #[doc = r#"For objects of version 3 or newer, this request determines the"#]
            #[doc = r#"final result of the drag-and-drop operation. If the end result"#]
            #[doc = r#"is that no mime types were accepted, the drag-and-drop operation"#]
            #[doc = r#"will be cancelled and the corresponding drag source will receive"#]
            #[doc = r#"wl_data_source.cancelled. Clients may still use this event in"#]
            #[doc = r#"conjunction with wl_data_source.action for feedback."#]
            async fn r#accept(
                client: &mut crate::Client,
                r#serial: u32,
                r#mime_type: Option<String>,
            ) -> crate::Result<()>;
            #[doc = r#"To transfer the offered data, the client issues this request"#]
            #[doc = r#"and indicates the mime type it wants to receive.  The transfer"#]
            #[doc = r#"happens through the passed file descriptor (typically created"#]
            #[doc = r#"with the pipe system call).  The source client writes the data"#]
            #[doc = r#"in the mime type representation requested and then closes the"#]
            #[doc = r#"file descriptor."#]
            #[doc = r#""#]
            #[doc = r#"The receiving client reads from the read end of the pipe until"#]
            #[doc = r#"EOF and then closes its end, at which point the transfer is"#]
            #[doc = r#"complete."#]
            #[doc = r#""#]
            #[doc = r#"This request may happen multiple times for different mime types,"#]
            #[doc = r#"both before and after wl_data_device.drop. Drag-and-drop destination"#]
            #[doc = r#"clients may preemptively fetch data or examine it more closely to"#]
            #[doc = r#"determine acceptance."#]
            async fn r#receive(
                client: &mut crate::Client,
                r#mime_type: String,
                r#fd: std::os::fd::RawFd,
            ) -> crate::Result<()>;
            #[doc = r#"Destroy the data offer."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Notifies the compositor that the drag destination successfully"#]
            #[doc = r#"finished the drag-and-drop operation."#]
            #[doc = r#""#]
            #[doc = r#"Upon receiving this request, the compositor will emit"#]
            #[doc = r#"wl_data_source.dnd_finished on the drag source client."#]
            #[doc = r#""#]
            #[doc = r#"It is a client error to perform other requests than"#]
            #[doc = r#"wl_data_offer.destroy after this one. It is also an error to perform"#]
            #[doc = r#"this request after a NULL mime type has been set in"#]
            #[doc = r#"wl_data_offer.accept or no action was received through"#]
            #[doc = r#"wl_data_offer.action."#]
            #[doc = r#""#]
            #[doc = r#"If wl_data_offer.finish request is received for a non drag and drop"#]
            #[doc = r#"operation, the invalid_finish protocol error is raised."#]
            async fn r#finish(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Sets the actions that the destination side client supports for"#]
            #[doc = r#"this operation. This request may trigger the emission of"#]
            #[doc = r#"wl_data_source.action and wl_data_offer.action events if the compositor"#]
            #[doc = r#"needs to change the selected action."#]
            #[doc = r#""#]
            #[doc = r#"This request can be called multiple times throughout the"#]
            #[doc = r#"drag-and-drop operation, typically in response to wl_data_device.enter"#]
            #[doc = r#"or wl_data_device.motion events."#]
            #[doc = r#""#]
            #[doc = r#"This request determines the final result of the drag-and-drop"#]
            #[doc = r#"operation. If the end result is that no action is accepted,"#]
            #[doc = r#"the drag source will receive wl_data_source.cancelled."#]
            #[doc = r#""#]
            #[doc = r#"The dnd_actions argument must contain only values expressed in the"#]
            #[doc = r#"wl_data_device_manager.dnd_actions enum, and the preferred_action"#]
            #[doc = r#"argument must only contain one of those values set, otherwise it"#]
            #[doc = r#"will result in a protocol error."#]
            #[doc = r#""#]
            #[doc = r#"While managing an "ask" action, the destination drag-and-drop client"#]
            #[doc = r#"may perform further wl_data_offer.receive requests, and is expected"#]
            #[doc = r#"to perform one last wl_data_offer.set_actions request with a preferred"#]
            #[doc = r#"action other than "ask" (and optionally wl_data_offer.accept) before"#]
            #[doc = r#"requesting wl_data_offer.finish, in order to convey the action selected"#]
            #[doc = r#"by the user. If the preferred action is not in the"#]
            #[doc = r#"wl_data_offer.source_actions mask, an error will be raised."#]
            #[doc = r#""#]
            #[doc = r#"If the "ask" action is dismissed (e.g. user cancellation), the client"#]
            #[doc = r#"is expected to perform wl_data_offer.destroy right away."#]
            #[doc = r#""#]
            #[doc = r#"This request can only be made on drag-and-drop offers, a protocol error"#]
            #[doc = r#"will be raised otherwise."#]
            async fn r#set_actions(
                client: &mut crate::Client,
                r#dnd_actions: u32,
                r#preferred_action: u32,
            ) -> crate::Result<()>;
            #[doc = r#"Sent immediately after creating the wl_data_offer object.  One"#]
            #[doc = r#"event per offered mime type."#]
            async fn r#offer(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#mime_type: String,
            ) -> crate::Result<()> {
                tracing::debug!("wl_data_offer -> offer");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_string(Some(mime_type))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event indicates the actions offered by the data source. It"#]
            #[doc = r#"will be sent immediately after creating the wl_data_offer object,"#]
            #[doc = r#"or anytime the source side changes its offered actions through"#]
            #[doc = r#"wl_data_source.set_actions."#]
            async fn r#source_actions(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#source_actions: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_data_offer -> source_actions");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(source_actions)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event indicates the action selected by the compositor after"#]
            #[doc = r#"matching the source/destination side actions. Only one action (or"#]
            #[doc = r#"none) will be offered here."#]
            #[doc = r#""#]
            #[doc = r#"This event can be emitted multiple times during the drag-and-drop"#]
            #[doc = r#"operation in response to destination side action changes through"#]
            #[doc = r#"wl_data_offer.set_actions."#]
            #[doc = r#""#]
            #[doc = r#"This event will no longer be emitted after wl_data_device.drop"#]
            #[doc = r#"happened on the drag-and-drop destination, the client must"#]
            #[doc = r#"honor the last action received, or the last preferred one set"#]
            #[doc = r#"through wl_data_offer.set_actions when handling an "ask" action."#]
            #[doc = r#""#]
            #[doc = r#"Compositors may also change the selected action on the fly, mainly"#]
            #[doc = r#"in response to keyboard modifier changes during the drag-and-drop"#]
            #[doc = r#"operation."#]
            #[doc = r#""#]
            #[doc = r#"The most recent action received is always the valid one. Prior to"#]
            #[doc = r#"receiving wl_data_device.drop, the chosen action may change (e.g."#]
            #[doc = r#"due to keyboard modifiers being pressed). At the time of receiving"#]
            #[doc = r#"wl_data_device.drop the drag-and-drop destination must honor the"#]
            #[doc = r#"last action received."#]
            #[doc = r#""#]
            #[doc = r#"Action changes may still happen after wl_data_device.drop,"#]
            #[doc = r#"especially on "ask" actions, where the drag-and-drop destination"#]
            #[doc = r#"may choose another action afterwards. Action changes happening"#]
            #[doc = r#"at this stage are always the result of inter-client negotiation, the"#]
            #[doc = r#"compositor shall no longer be able to induce a different action."#]
            #[doc = r#""#]
            #[doc = r#"Upon "ask" actions, it is expected that the drag-and-drop destination"#]
            #[doc = r#"may potentially choose a different action and/or mime type,"#]
            #[doc = r#"based on wl_data_offer.source_actions and finally chosen by the"#]
            #[doc = r#"user (e.g. popping up a menu with the available options). The"#]
            #[doc = r#"final wl_data_offer.set_actions and wl_data_offer.accept requests"#]
            #[doc = r#"must happen before the call to wl_data_offer.finish."#]
            async fn r#action(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#dnd_action: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_data_offer -> action");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(dnd_action)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod wl_data_source {
        enum r#Error {}
        #[doc = r#"The wl_data_source object is the source side of a wl_data_offer."#]
        #[doc = r#"It is created by the source client in a data transfer and"#]
        #[doc = r#"provides a way to describe the offered data and a way to respond"#]
        #[doc = r#"to requests to transfer the data."#]
        pub trait r#WlDataSource {
            const INTERFACE: &'static str = "wl_data_source";
            const VERSION: u32 = 3;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_data_source -> offer");
                        Self::r#offer(
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1 => {
                        tracing::debug!("wl_data_source -> destroy");
                        Self::r#destroy(client).await
                    }
                    2 => {
                        tracing::debug!("wl_data_source -> set_actions");
                        Self::r#set_actions(client, message.uint()?).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"This request adds a mime type to the set of mime types"#]
            #[doc = r#"advertised to targets.  Can be called several times to offer"#]
            #[doc = r#"multiple types."#]
            async fn r#offer(client: &mut crate::Client, r#mime_type: String) -> crate::Result<()>;
            #[doc = r#"Destroy the data source."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Sets the actions that the source side client supports for this"#]
            #[doc = r#"operation. This request may trigger wl_data_source.action and"#]
            #[doc = r#"wl_data_offer.action events if the compositor needs to change the"#]
            #[doc = r#"selected action."#]
            #[doc = r#""#]
            #[doc = r#"The dnd_actions argument must contain only values expressed in the"#]
            #[doc = r#"wl_data_device_manager.dnd_actions enum, otherwise it will result"#]
            #[doc = r#"in a protocol error."#]
            #[doc = r#""#]
            #[doc = r#"This request must be made once only, and can only be made on sources"#]
            #[doc = r#"used in drag-and-drop, so it must be performed before"#]
            #[doc = r#"wl_data_device.start_drag. Attempting to use the source other than"#]
            #[doc = r#"for drag-and-drop will raise a protocol error."#]
            async fn r#set_actions(
                client: &mut crate::Client,
                r#dnd_actions: u32,
            ) -> crate::Result<()>;
            #[doc = r#"Sent when a target accepts pointer_focus or motion events.  If"#]
            #[doc = r#"a target does not accept any of the offered types, type is NULL."#]
            #[doc = r#""#]
            #[doc = r#"Used for feedback during drag-and-drop."#]
            async fn r#target(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#mime_type: Option<String>,
            ) -> crate::Result<()> {
                tracing::debug!("wl_data_source -> target");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_string(mime_type)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Request for data from the client.  Send the data as the"#]
            #[doc = r#"specified mime type over the passed file descriptor, then"#]
            #[doc = r#"close it."#]
            async fn r#send(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#mime_type: String,
                r#fd: std::os::fd::RawFd,
            ) -> crate::Result<()> {
                tracing::debug!("wl_data_source -> send");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_string(Some(mime_type))
                    .put_int(fd)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This data source is no longer valid. There are several reasons why"#]
            #[doc = r#"this could happen:"#]
            #[doc = r#""#]
            #[doc = r#"- The data source has been replaced by another data source."#]
            #[doc = r#"- The drag-and-drop operation was performed, but the drop destination"#]
            #[doc = r#"did not accept any of the mime types offered through"#]
            #[doc = r#"wl_data_source.target."#]
            #[doc = r#"- The drag-and-drop operation was performed, but the drop destination"#]
            #[doc = r#"did not select any of the actions present in the mask offered through"#]
            #[doc = r#"wl_data_source.action."#]
            #[doc = r#"- The drag-and-drop operation was performed but didn't happen over a"#]
            #[doc = r#"surface."#]
            #[doc = r#"- The compositor cancelled the drag-and-drop operation (e.g. compositor"#]
            #[doc = r#"dependent timeouts to avoid stale drag-and-drop transfers)."#]
            #[doc = r#""#]
            #[doc = r#"The client should clean up and destroy this data source."#]
            #[doc = r#""#]
            #[doc = r#"For objects of version 2 or older, wl_data_source.cancelled will"#]
            #[doc = r#"only be emitted if the data source was replaced by another data"#]
            #[doc = r#"source."#]
            async fn r#cancelled(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("wl_data_source -> cancelled");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"The user performed the drop action. This event does not indicate"#]
            #[doc = r#"acceptance, wl_data_source.cancelled may still be emitted afterwards"#]
            #[doc = r#"if the drop destination does not accept any mime type."#]
            #[doc = r#""#]
            #[doc = r#"However, this event might however not be received if the compositor"#]
            #[doc = r#"cancelled the drag-and-drop operation before this event could happen."#]
            #[doc = r#""#]
            #[doc = r#"Note that the data_source may still be used in the future and should"#]
            #[doc = r#"not be destroyed here."#]
            async fn r#dnd_drop_performed(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("wl_data_source -> dnd_drop_performed");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 3, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"The drop destination finished interoperating with this data"#]
            #[doc = r#"source, so the client is now free to destroy this data source and"#]
            #[doc = r#"free all associated data."#]
            #[doc = r#""#]
            #[doc = r#"If the action used to perform the operation was "move", the"#]
            #[doc = r#"source can now delete the transferred data."#]
            async fn r#dnd_finished(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("wl_data_source -> dnd_finished");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 4, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event indicates the action selected by the compositor after"#]
            #[doc = r#"matching the source/destination side actions. Only one action (or"#]
            #[doc = r#"none) will be offered here."#]
            #[doc = r#""#]
            #[doc = r#"This event can be emitted multiple times during the drag-and-drop"#]
            #[doc = r#"operation, mainly in response to destination side changes through"#]
            #[doc = r#"wl_data_offer.set_actions, and as the data device enters/leaves"#]
            #[doc = r#"surfaces."#]
            #[doc = r#""#]
            #[doc = r#"It is only possible to receive this event after"#]
            #[doc = r#"wl_data_source.dnd_drop_performed if the drag-and-drop operation"#]
            #[doc = r#"ended in an "ask" action, in which case the final wl_data_source.action"#]
            #[doc = r#"event will happen immediately before wl_data_source.dnd_finished."#]
            #[doc = r#""#]
            #[doc = r#"Compositors may also change the selected action on the fly, mainly"#]
            #[doc = r#"in response to keyboard modifier changes during the drag-and-drop"#]
            #[doc = r#"operation."#]
            #[doc = r#""#]
            #[doc = r#"The most recent action received is always the valid one. The chosen"#]
            #[doc = r#"action may change alongside negotiation (e.g. an "ask" action can turn"#]
            #[doc = r#"into a "move" operation), so the effects of the final action must"#]
            #[doc = r#"always be applied in wl_data_offer.dnd_finished."#]
            #[doc = r#""#]
            #[doc = r#"Clients can trigger cursor surface changes from this point, so"#]
            #[doc = r#"they reflect the current action."#]
            async fn r#action(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#dnd_action: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_data_source -> action");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(dnd_action)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 5, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod wl_data_device {
        enum r#Error {}
        #[doc = r#"There is one wl_data_device per seat which can be obtained"#]
        #[doc = r#"from the global wl_data_device_manager singleton."#]
        #[doc = r#""#]
        #[doc = r#"A wl_data_device provides access to inter-client data transfer"#]
        #[doc = r#"mechanisms such as copy-and-paste and drag-and-drop."#]
        pub trait r#WlDataDevice {
            const INTERFACE: &'static str = "wl_data_device";
            const VERSION: u32 = 3;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_data_device -> start_drag");
                        Self::r#start_drag(
                            client,
                            message.object()?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.object()?,
                            message.uint()?,
                        )
                        .await
                    }
                    1 => {
                        tracing::debug!("wl_data_device -> set_selection");
                        Self::r#set_selection(client, message.object()?, message.uint()?).await
                    }
                    2 => {
                        tracing::debug!("wl_data_device -> release");
                        Self::r#release(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"This request asks the compositor to start a drag-and-drop"#]
            #[doc = r#"operation on behalf of the client."#]
            #[doc = r#""#]
            #[doc = r#"The source argument is the data source that provides the data"#]
            #[doc = r#"for the eventual data transfer. If source is NULL, enter, leave"#]
            #[doc = r#"and motion events are sent only to the client that initiated the"#]
            #[doc = r#"drag and the client is expected to handle the data passing"#]
            #[doc = r#"internally. If source is destroyed, the drag-and-drop session will be"#]
            #[doc = r#"cancelled."#]
            #[doc = r#""#]
            #[doc = r#"The origin surface is the surface where the drag originates and"#]
            #[doc = r#"the client must have an active implicit grab that matches the"#]
            #[doc = r#"serial."#]
            #[doc = r#""#]
            #[doc = r#"The icon surface is an optional (can be NULL) surface that"#]
            #[doc = r#"provides an icon to be moved around with the cursor.  Initially,"#]
            #[doc = r#"the top-left corner of the icon surface is placed at the cursor"#]
            #[doc = r#"hotspot, but subsequent wl_surface.offset requests can move the"#]
            #[doc = r#"relative position. Attach requests must be confirmed with"#]
            #[doc = r#"wl_surface.commit as usual. The icon surface is given the role of"#]
            #[doc = r#"a drag-and-drop icon. If the icon surface already has another role,"#]
            #[doc = r#"it raises a protocol error."#]
            #[doc = r#""#]
            #[doc = r#"The input region is ignored for wl_surfaces with the role of a"#]
            #[doc = r#"drag-and-drop icon."#]
            #[doc = r#""#]
            #[doc = r#"The given source may not be used in any further set_selection or"#]
            #[doc = r#"start_drag requests. Attempting to reuse a previously-used source"#]
            #[doc = r#"may send a used_source error."#]
            async fn r#start_drag(
                client: &mut crate::Client,
                r#source: Option<crate::wire::ObjectId>,
                r#origin: crate::wire::ObjectId,
                r#icon: Option<crate::wire::ObjectId>,
                r#serial: u32,
            ) -> crate::Result<()>;
            #[doc = r#"This request asks the compositor to set the selection"#]
            #[doc = r#"to the data from the source on behalf of the client."#]
            #[doc = r#""#]
            #[doc = r#"To unset the selection, set the source to NULL."#]
            #[doc = r#""#]
            #[doc = r#"The given source may not be used in any further set_selection or"#]
            #[doc = r#"start_drag requests. Attempting to reuse a previously-used source"#]
            #[doc = r#"may send a used_source error."#]
            async fn r#set_selection(
                client: &mut crate::Client,
                r#source: Option<crate::wire::ObjectId>,
                r#serial: u32,
            ) -> crate::Result<()>;
            #[doc = r#"This request destroys the data device."#]
            async fn r#release(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"The data_offer event introduces a new wl_data_offer object,"#]
            #[doc = r#"which will subsequently be used in either the"#]
            #[doc = r#"data_device.enter event (for drag-and-drop) or the"#]
            #[doc = r#"data_device.selection event (for selections).  Immediately"#]
            #[doc = r#"following the data_device.data_offer event, the new data_offer"#]
            #[doc = r#"object will send out data_offer.offer events to describe the"#]
            #[doc = r#"mime types it offers."#]
            async fn r#data_offer(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
            ) -> crate::Result<()> {
                tracing::debug!("wl_data_device -> data_offer");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event is sent when an active drag-and-drop pointer enters"#]
            #[doc = r#"a surface owned by the client.  The position of the pointer at"#]
            #[doc = r#"enter time is provided by the x and y arguments, in surface-local"#]
            #[doc = r#"coordinates."#]
            async fn r#enter(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
                r#surface: crate::wire::ObjectId,
                r#x: crate::wire::Fixed,
                r#y: crate::wire::Fixed,
                r#id: Option<crate::wire::ObjectId>,
            ) -> crate::Result<()> {
                tracing::debug!("wl_data_device -> enter");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(Some(surface))
                    .put_fixed(x)
                    .put_fixed(y)
                    .put_object(id)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event is sent when the drag-and-drop pointer leaves the"#]
            #[doc = r#"surface and the session ends.  The client must destroy the"#]
            #[doc = r#"wl_data_offer introduced at enter time at this point."#]
            async fn r#leave(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("wl_data_device -> leave");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event is sent when the drag-and-drop pointer moves within"#]
            #[doc = r#"the currently focused surface. The new position of the pointer"#]
            #[doc = r#"is provided by the x and y arguments, in surface-local"#]
            #[doc = r#"coordinates."#]
            async fn r#motion(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#time: u32,
                r#x: crate::wire::Fixed,
                r#y: crate::wire::Fixed,
            ) -> crate::Result<()> {
                tracing::debug!("wl_data_device -> motion");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_fixed(x)
                    .put_fixed(y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 3, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"The event is sent when a drag-and-drop operation is ended"#]
            #[doc = r#"because the implicit grab is removed."#]
            #[doc = r#""#]
            #[doc = r#"The drag-and-drop destination is expected to honor the last action"#]
            #[doc = r#"received through wl_data_offer.action, if the resulting action is"#]
            #[doc = r#""copy" or "move", the destination can still perform"#]
            #[doc = r#"wl_data_offer.receive requests, and is expected to end all"#]
            #[doc = r#"transfers with a wl_data_offer.finish request."#]
            #[doc = r#""#]
            #[doc = r#"If the resulting action is "ask", the action will not be considered"#]
            #[doc = r#"final. The drag-and-drop destination is expected to perform one last"#]
            #[doc = r#"wl_data_offer.set_actions request, or wl_data_offer.destroy in order"#]
            #[doc = r#"to cancel the operation."#]
            async fn r#drop(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("wl_data_device -> drop");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 4, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"The selection event is sent out to notify the client of a new"#]
            #[doc = r#"wl_data_offer for the selection for this device.  The"#]
            #[doc = r#"data_device.data_offer and the data_offer.offer events are"#]
            #[doc = r#"sent out immediately before this event to introduce the data"#]
            #[doc = r#"offer object.  The selection event is sent to a client"#]
            #[doc = r#"immediately before receiving keyboard focus and when a new"#]
            #[doc = r#"selection is set while the client has keyboard focus.  The"#]
            #[doc = r#"data_offer is valid until a new data_offer or NULL is received"#]
            #[doc = r#"or until the client loses keyboard focus.  Switching surface with"#]
            #[doc = r#"keyboard focus within the same client doesn't mean a new selection"#]
            #[doc = r#"will be sent.  The client must destroy the previous selection"#]
            #[doc = r#"data_offer, if any, upon receiving this event."#]
            async fn r#selection(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#id: Option<crate::wire::ObjectId>,
            ) -> crate::Result<()> {
                tracing::debug!("wl_data_device -> selection");
                let payload = crate::wire::PayloadBuilder::new().put_object(id).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 5, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod wl_data_device_manager {
        enum r#DndAction {}
        #[doc = r#"The wl_data_device_manager is a singleton global object that"#]
        #[doc = r#"provides access to inter-client data transfer mechanisms such as"#]
        #[doc = r#"copy-and-paste and drag-and-drop.  These mechanisms are tied to"#]
        #[doc = r#"a wl_seat and this interface lets a client get a wl_data_device"#]
        #[doc = r#"corresponding to a wl_seat."#]
        #[doc = r#""#]
        #[doc = r#"Depending on the version bound, the objects created from the bound"#]
        #[doc = r#"wl_data_device_manager object will have different requirements for"#]
        #[doc = r#"functioning properly. See wl_data_source.set_actions,"#]
        #[doc = r#"wl_data_offer.accept and wl_data_offer.finish for details."#]
        pub trait r#WlDataDeviceManager {
            const INTERFACE: &'static str = "wl_data_device_manager";
            const VERSION: u32 = 3;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_data_device_manager -> create_data_source");
                        Self::r#create_data_source(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1 => {
                        tracing::debug!("wl_data_device_manager -> get_data_device");
                        Self::r#get_data_device(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Create a new data source."#]
            async fn r#create_data_source(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"Create a new data device for a given seat."#]
            async fn r#get_data_device(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
                r#seat: crate::wire::ObjectId,
            ) -> crate::Result<()>;
        }
    }
    pub mod wl_shell {
        enum r#Error {}
        #[doc = r#"This interface is implemented by servers that provide"#]
        #[doc = r#"desktop-style user interfaces."#]
        #[doc = r#""#]
        #[doc = r#"It allows clients to associate a wl_shell_surface with"#]
        #[doc = r#"a basic surface."#]
        #[doc = r#""#]
        #[doc = r#"Note! This protocol is deprecated and not intended for production use."#]
        #[doc = r#"For desktop-style user interfaces, use xdg_shell. Compositors and clients"#]
        #[doc = r#"should not implement this interface."#]
        pub trait r#WlShell {
            const INTERFACE: &'static str = "wl_shell";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_shell -> get_shell_surface");
                        Self::r#get_shell_surface(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Create a shell surface for an existing surface. This gives"#]
            #[doc = r#"the wl_surface the role of a shell surface. If the wl_surface"#]
            #[doc = r#"already has another role, it raises a protocol error."#]
            #[doc = r#""#]
            #[doc = r#"Only one shell surface can be associated with a given surface."#]
            async fn r#get_shell_surface(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
                r#surface: crate::wire::ObjectId,
            ) -> crate::Result<()>;
        }
    }
    pub mod wl_shell_surface {
        enum r#Resize {}
        enum r#Transient {}
        enum r#FullscreenMethod {}
        #[doc = r#"An interface that may be implemented by a wl_surface, for"#]
        #[doc = r#"implementations that provide a desktop-style user interface."#]
        #[doc = r#""#]
        #[doc = r#"It provides requests to treat surfaces like toplevel, fullscreen"#]
        #[doc = r#"or popup windows, move, resize or maximize them, associate"#]
        #[doc = r#"metadata like title and class, etc."#]
        #[doc = r#""#]
        #[doc = r#"On the server side the object is automatically destroyed when"#]
        #[doc = r#"the related wl_surface is destroyed. On the client side,"#]
        #[doc = r#"wl_shell_surface_destroy() must be called before destroying"#]
        #[doc = r#"the wl_surface object."#]
        pub trait r#WlShellSurface {
            const INTERFACE: &'static str = "wl_shell_surface";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_shell_surface -> pong");
                        Self::r#pong(client, message.uint()?).await
                    }
                    1 => {
                        tracing::debug!("wl_shell_surface -> move");
                        Self::r#move(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    2 => {
                        tracing::debug!("wl_shell_surface -> resize");
                        Self::r#resize(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                            message.uint()?,
                        )
                        .await
                    }
                    3 => {
                        tracing::debug!("wl_shell_surface -> set_toplevel");
                        Self::r#set_toplevel(client).await
                    }
                    4 => {
                        tracing::debug!("wl_shell_surface -> set_transient");
                        Self::r#set_transient(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                            message.int()?,
                            message.uint()?,
                        )
                        .await
                    }
                    5 => {
                        tracing::debug!("wl_shell_surface -> set_fullscreen");
                        Self::r#set_fullscreen(
                            client,
                            message.uint()?,
                            message.uint()?,
                            message.object()?,
                        )
                        .await
                    }
                    6 => {
                        tracing::debug!("wl_shell_surface -> set_popup");
                        Self::r#set_popup(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                            message.int()?,
                            message.uint()?,
                        )
                        .await
                    }
                    7 => {
                        tracing::debug!("wl_shell_surface -> set_maximized");
                        Self::r#set_maximized(client, message.object()?).await
                    }
                    8 => {
                        tracing::debug!("wl_shell_surface -> set_title");
                        Self::r#set_title(
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    9 => {
                        tracing::debug!("wl_shell_surface -> set_class");
                        Self::r#set_class(
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"A client must respond to a ping event with a pong request or"#]
            #[doc = r#"the client may be deemed unresponsive."#]
            async fn r#pong(client: &mut crate::Client, r#serial: u32) -> crate::Result<()>;
            #[doc = r#"Start a pointer-driven move of the surface."#]
            #[doc = r#""#]
            #[doc = r#"This request must be used in response to a button press event."#]
            #[doc = r#"The server may ignore move requests depending on the state of"#]
            #[doc = r#"the surface (e.g. fullscreen or maximized)."#]
            async fn r#move(
                client: &mut crate::Client,
                r#seat: crate::wire::ObjectId,
                r#serial: u32,
            ) -> crate::Result<()>;
            #[doc = r#"Start a pointer-driven resizing of the surface."#]
            #[doc = r#""#]
            #[doc = r#"This request must be used in response to a button press event."#]
            #[doc = r#"The server may ignore resize requests depending on the state of"#]
            #[doc = r#"the surface (e.g. fullscreen or maximized)."#]
            async fn r#resize(
                client: &mut crate::Client,
                r#seat: crate::wire::ObjectId,
                r#serial: u32,
                r#edges: u32,
            ) -> crate::Result<()>;
            #[doc = r#"Map the surface as a toplevel surface."#]
            #[doc = r#""#]
            #[doc = r#"A toplevel surface is not fullscreen, maximized or transient."#]
            async fn r#set_toplevel(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Map the surface relative to an existing surface."#]
            #[doc = r#""#]
            #[doc = r#"The x and y arguments specify the location of the upper left"#]
            #[doc = r#"corner of the surface relative to the upper left corner of the"#]
            #[doc = r#"parent surface, in surface-local coordinates."#]
            #[doc = r#""#]
            #[doc = r#"The flags argument controls details of the transient behaviour."#]
            async fn r#set_transient(
                client: &mut crate::Client,
                r#parent: crate::wire::ObjectId,
                r#x: i32,
                r#y: i32,
                r#flags: u32,
            ) -> crate::Result<()>;
            #[doc = r#"Map the surface as a fullscreen surface."#]
            #[doc = r#""#]
            #[doc = r#"If an output parameter is given then the surface will be made"#]
            #[doc = r#"fullscreen on that output. If the client does not specify the"#]
            #[doc = r#"output then the compositor will apply its policy - usually"#]
            #[doc = r#"choosing the output on which the surface has the biggest surface"#]
            #[doc = r#"area."#]
            #[doc = r#""#]
            #[doc = r#"The client may specify a method to resolve a size conflict"#]
            #[doc = r#"between the output size and the surface size - this is provided"#]
            #[doc = r#"through the method parameter."#]
            #[doc = r#""#]
            #[doc = r#"The framerate parameter is used only when the method is set"#]
            #[doc = r#"to "driver", to indicate the preferred framerate. A value of 0"#]
            #[doc = r#"indicates that the client does not care about framerate.  The"#]
            #[doc = r#"framerate is specified in mHz, that is framerate of 60000 is 60Hz."#]
            #[doc = r#""#]
            #[doc = r#"A method of "scale" or "driver" implies a scaling operation of"#]
            #[doc = r#"the surface, either via a direct scaling operation or a change of"#]
            #[doc = r#"the output mode. This will override any kind of output scaling, so"#]
            #[doc = r#"that mapping a surface with a buffer size equal to the mode can"#]
            #[doc = r#"fill the screen independent of buffer_scale."#]
            #[doc = r#""#]
            #[doc = r#"A method of "fill" means we don't scale up the buffer, however"#]
            #[doc = r#"any output scale is applied. This means that you may run into"#]
            #[doc = r#"an edge case where the application maps a buffer with the same"#]
            #[doc = r#"size of the output mode but buffer_scale 1 (thus making a"#]
            #[doc = r#"surface larger than the output). In this case it is allowed to"#]
            #[doc = r#"downscale the results to fit the screen."#]
            #[doc = r#""#]
            #[doc = r#"The compositor must reply to this request with a configure event"#]
            #[doc = r#"with the dimensions for the output on which the surface will"#]
            #[doc = r#"be made fullscreen."#]
            async fn r#set_fullscreen(
                client: &mut crate::Client,
                r#method: u32,
                r#framerate: u32,
                r#output: Option<crate::wire::ObjectId>,
            ) -> crate::Result<()>;
            #[doc = r#"Map the surface as a popup."#]
            #[doc = r#""#]
            #[doc = r#"A popup surface is a transient surface with an added pointer"#]
            #[doc = r#"grab."#]
            #[doc = r#""#]
            #[doc = r#"An existing implicit grab will be changed to owner-events mode,"#]
            #[doc = r#"and the popup grab will continue after the implicit grab ends"#]
            #[doc = r#"(i.e. releasing the mouse button does not cause the popup to"#]
            #[doc = r#"be unmapped)."#]
            #[doc = r#""#]
            #[doc = r#"The popup grab continues until the window is destroyed or a"#]
            #[doc = r#"mouse button is pressed in any other client's window. A click"#]
            #[doc = r#"in any of the client's surfaces is reported as normal, however,"#]
            #[doc = r#"clicks in other clients' surfaces will be discarded and trigger"#]
            #[doc = r#"the callback."#]
            #[doc = r#""#]
            #[doc = r#"The x and y arguments specify the location of the upper left"#]
            #[doc = r#"corner of the surface relative to the upper left corner of the"#]
            #[doc = r#"parent surface, in surface-local coordinates."#]
            async fn r#set_popup(
                client: &mut crate::Client,
                r#seat: crate::wire::ObjectId,
                r#serial: u32,
                r#parent: crate::wire::ObjectId,
                r#x: i32,
                r#y: i32,
                r#flags: u32,
            ) -> crate::Result<()>;
            #[doc = r#"Map the surface as a maximized surface."#]
            #[doc = r#""#]
            #[doc = r#"If an output parameter is given then the surface will be"#]
            #[doc = r#"maximized on that output. If the client does not specify the"#]
            #[doc = r#"output then the compositor will apply its policy - usually"#]
            #[doc = r#"choosing the output on which the surface has the biggest surface"#]
            #[doc = r#"area."#]
            #[doc = r#""#]
            #[doc = r#"The compositor will reply with a configure event telling"#]
            #[doc = r#"the expected new surface size. The operation is completed"#]
            #[doc = r#"on the next buffer attach to this surface."#]
            #[doc = r#""#]
            #[doc = r#"A maximized surface typically fills the entire output it is"#]
            #[doc = r#"bound to, except for desktop elements such as panels. This is"#]
            #[doc = r#"the main difference between a maximized shell surface and a"#]
            #[doc = r#"fullscreen shell surface."#]
            #[doc = r#""#]
            #[doc = r#"The details depend on the compositor implementation."#]
            async fn r#set_maximized(
                client: &mut crate::Client,
                r#output: Option<crate::wire::ObjectId>,
            ) -> crate::Result<()>;
            #[doc = r#"Set a short title for the surface."#]
            #[doc = r#""#]
            #[doc = r#"This string may be used to identify the surface in a task bar,"#]
            #[doc = r#"window list, or other user interface elements provided by the"#]
            #[doc = r#"compositor."#]
            #[doc = r#""#]
            #[doc = r#"The string must be encoded in UTF-8."#]
            async fn r#set_title(client: &mut crate::Client, r#title: String) -> crate::Result<()>;
            #[doc = r#"Set a class for the surface."#]
            #[doc = r#""#]
            #[doc = r#"The surface class identifies the general class of applications"#]
            #[doc = r#"to which the surface belongs. A common convention is to use the"#]
            #[doc = r#"file name (or the full path if it is a non-standard location) of"#]
            #[doc = r#"the application's .desktop file as the class."#]
            async fn r#set_class(client: &mut crate::Client, r#class: String) -> crate::Result<()>;
            #[doc = r#"Ping a client to check if it is receiving events and sending"#]
            #[doc = r#"requests. A client is expected to reply with a pong request."#]
            async fn r#ping(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_shell_surface -> ping");
                let payload = crate::wire::PayloadBuilder::new().put_uint(serial).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"The configure event asks the client to resize its surface."#]
            #[doc = r#""#]
            #[doc = r#"The size is a hint, in the sense that the client is free to"#]
            #[doc = r#"ignore it if it doesn't resize, pick a smaller size (to"#]
            #[doc = r#"satisfy aspect ratio or resize in steps of NxM pixels)."#]
            #[doc = r#""#]
            #[doc = r#"The edges parameter provides a hint about how the surface"#]
            #[doc = r#"was resized. The client may use this information to decide"#]
            #[doc = r#"how to adjust its content to the new size (e.g. a scrolling"#]
            #[doc = r#"area might adjust its content position to leave the viewable"#]
            #[doc = r#"content unmoved)."#]
            #[doc = r#""#]
            #[doc = r#"The client is free to dismiss all but the last configure"#]
            #[doc = r#"event it received."#]
            #[doc = r#""#]
            #[doc = r#"The width and height arguments specify the size of the window"#]
            #[doc = r#"in surface-local coordinates."#]
            async fn r#configure(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#edges: u32,
                r#width: i32,
                r#height: i32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_shell_surface -> configure");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(edges)
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"The popup_done event is sent out when a popup grab is broken,"#]
            #[doc = r#"that is, when the user clicks a surface that doesn't belong"#]
            #[doc = r#"to the client owning the popup surface."#]
            async fn r#popup_done(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("wl_shell_surface -> popup_done");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod wl_surface {
        enum r#Error {}
        #[doc = r#"A surface is a rectangular area that may be displayed on zero"#]
        #[doc = r#"or more outputs, and shown any number of times at the compositor's"#]
        #[doc = r#"discretion. They can present wl_buffers, receive user input, and"#]
        #[doc = r#"define a local coordinate system."#]
        #[doc = r#""#]
        #[doc = r#"The size of a surface (and relative positions on it) is described"#]
        #[doc = r#"in surface-local coordinates, which may differ from the buffer"#]
        #[doc = r#"coordinates of the pixel content, in case a buffer_transform"#]
        #[doc = r#"or a buffer_scale is used."#]
        #[doc = r#""#]
        #[doc = r#"A surface without a "role" is fairly useless: a compositor does"#]
        #[doc = r#"not know where, when or how to present it. The role is the"#]
        #[doc = r#"purpose of a wl_surface. Examples of roles are a cursor for a"#]
        #[doc = r#"pointer (as set by wl_pointer.set_cursor), a drag icon"#]
        #[doc = r#"(wl_data_device.start_drag), a sub-surface"#]
        #[doc = r#"(wl_subcompositor.get_subsurface), and a window as defined by a"#]
        #[doc = r#"shell protocol (e.g. wl_shell.get_shell_surface)."#]
        #[doc = r#""#]
        #[doc = r#"A surface can have only one role at a time. Initially a"#]
        #[doc = r#"wl_surface does not have a role. Once a wl_surface is given a"#]
        #[doc = r#"role, it is set permanently for the whole lifetime of the"#]
        #[doc = r#"wl_surface object. Giving the current role again is allowed,"#]
        #[doc = r#"unless explicitly forbidden by the relevant interface"#]
        #[doc = r#"specification."#]
        #[doc = r#""#]
        #[doc = r#"Surface roles are given by requests in other interfaces such as"#]
        #[doc = r#"wl_pointer.set_cursor. The request should explicitly mention"#]
        #[doc = r#"that this request gives a role to a wl_surface. Often, this"#]
        #[doc = r#"request also creates a new protocol object that represents the"#]
        #[doc = r#"role and adds additional functionality to wl_surface. When a"#]
        #[doc = r#"client wants to destroy a wl_surface, they must destroy this role"#]
        #[doc = r#"object before the wl_surface, otherwise a defunct_role_object error is"#]
        #[doc = r#"sent."#]
        #[doc = r#""#]
        #[doc = r#"Destroying the role object does not remove the role from the"#]
        #[doc = r#"wl_surface, but it may stop the wl_surface from "playing the role"."#]
        #[doc = r#"For instance, if a wl_subsurface object is destroyed, the wl_surface"#]
        #[doc = r#"it was created for will be unmapped and forget its position and"#]
        #[doc = r#"z-order. It is allowed to create a wl_subsurface for the same"#]
        #[doc = r#"wl_surface again, but it is not allowed to use the wl_surface as"#]
        #[doc = r#"a cursor (cursor is a different role than sub-surface, and role"#]
        #[doc = r#"switching is not allowed)."#]
        pub trait r#WlSurface {
            const INTERFACE: &'static str = "wl_surface";
            const VERSION: u32 = 6;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_surface -> destroy");
                        Self::r#destroy(client).await
                    }
                    1 => {
                        tracing::debug!("wl_surface -> attach");
                        Self::r#attach(client, message.object()?, message.int()?, message.int()?)
                            .await
                    }
                    2 => {
                        tracing::debug!("wl_surface -> damage");
                        Self::r#damage(
                            client,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    3 => {
                        tracing::debug!("wl_surface -> frame");
                        Self::r#frame(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    4 => {
                        tracing::debug!("wl_surface -> set_opaque_region");
                        Self::r#set_opaque_region(client, message.object()?).await
                    }
                    5 => {
                        tracing::debug!("wl_surface -> set_input_region");
                        Self::r#set_input_region(client, message.object()?).await
                    }
                    6 => {
                        tracing::debug!("wl_surface -> commit");
                        Self::r#commit(client).await
                    }
                    7 => {
                        tracing::debug!("wl_surface -> set_buffer_transform");
                        Self::r#set_buffer_transform(client, message.int()?).await
                    }
                    8 => {
                        tracing::debug!("wl_surface -> set_buffer_scale");
                        Self::r#set_buffer_scale(client, message.int()?).await
                    }
                    9 => {
                        tracing::debug!("wl_surface -> damage_buffer");
                        Self::r#damage_buffer(
                            client,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    10 => {
                        tracing::debug!("wl_surface -> offset");
                        Self::r#offset(client, message.int()?, message.int()?).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Deletes the surface and invalidates its object ID."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Set a buffer as the content of this surface."#]
            #[doc = r#""#]
            #[doc = r#"The new size of the surface is calculated based on the buffer"#]
            #[doc = r#"size transformed by the inverse buffer_transform and the"#]
            #[doc = r#"inverse buffer_scale. This means that at commit time the supplied"#]
            #[doc = r#"buffer size must be an integer multiple of the buffer_scale. If"#]
            #[doc = r#"that's not the case, an invalid_size error is sent."#]
            #[doc = r#""#]
            #[doc = r#"The x and y arguments specify the location of the new pending"#]
            #[doc = r#"buffer's upper left corner, relative to the current buffer's upper"#]
            #[doc = r#"left corner, in surface-local coordinates. In other words, the"#]
            #[doc = r#"x and y, combined with the new surface size define in which"#]
            #[doc = r#"directions the surface's size changes. Setting anything other than 0"#]
            #[doc = r#"as x and y arguments is discouraged, and should instead be replaced"#]
            #[doc = r#"with using the separate wl_surface.offset request."#]
            #[doc = r#""#]
            #[doc = r#"When the bound wl_surface version is 5 or higher, passing any"#]
            #[doc = r#"non-zero x or y is a protocol violation, and will result in an"#]
            #[doc = r#"'invalid_offset' error being raised. The x and y arguments are ignored"#]
            #[doc = r#"and do not change the pending state. To achieve equivalent semantics,"#]
            #[doc = r#"use wl_surface.offset."#]
            #[doc = r#""#]
            #[doc = r#"Surface contents are double-buffered state, see wl_surface.commit."#]
            #[doc = r#""#]
            #[doc = r#"The initial surface contents are void; there is no content."#]
            #[doc = r#"wl_surface.attach assigns the given wl_buffer as the pending"#]
            #[doc = r#"wl_buffer. wl_surface.commit makes the pending wl_buffer the new"#]
            #[doc = r#"surface contents, and the size of the surface becomes the size"#]
            #[doc = r#"calculated from the wl_buffer, as described above. After commit,"#]
            #[doc = r#"there is no pending buffer until the next attach."#]
            #[doc = r#""#]
            #[doc = r#"Committing a pending wl_buffer allows the compositor to read the"#]
            #[doc = r#"pixels in the wl_buffer. The compositor may access the pixels at"#]
            #[doc = r#"any time after the wl_surface.commit request. When the compositor"#]
            #[doc = r#"will not access the pixels anymore, it will send the"#]
            #[doc = r#"wl_buffer.release event. Only after receiving wl_buffer.release,"#]
            #[doc = r#"the client may reuse the wl_buffer. A wl_buffer that has been"#]
            #[doc = r#"attached and then replaced by another attach instead of committed"#]
            #[doc = r#"will not receive a release event, and is not used by the"#]
            #[doc = r#"compositor."#]
            #[doc = r#""#]
            #[doc = r#"If a pending wl_buffer has been committed to more than one wl_surface,"#]
            #[doc = r#"the delivery of wl_buffer.release events becomes undefined. A well"#]
            #[doc = r#"behaved client should not rely on wl_buffer.release events in this"#]
            #[doc = r#"case. Alternatively, a client could create multiple wl_buffer objects"#]
            #[doc = r#"from the same backing storage or use wp_linux_buffer_release."#]
            #[doc = r#""#]
            #[doc = r#"Destroying the wl_buffer after wl_buffer.release does not change"#]
            #[doc = r#"the surface contents. Destroying the wl_buffer before wl_buffer.release"#]
            #[doc = r#"is allowed as long as the underlying buffer storage isn't re-used (this"#]
            #[doc = r#"can happen e.g. on client process termination). However, if the client"#]
            #[doc = r#"destroys the wl_buffer before receiving the wl_buffer.release event and"#]
            #[doc = r#"mutates the underlying buffer storage, the surface contents become"#]
            #[doc = r#"undefined immediately."#]
            #[doc = r#""#]
            #[doc = r#"If wl_surface.attach is sent with a NULL wl_buffer, the"#]
            #[doc = r#"following wl_surface.commit will remove the surface content."#]
            #[doc = r#""#]
            #[doc = r#"If a pending wl_buffer has been destroyed, the result is not specified."#]
            #[doc = r#"Many compositors are known to remove the surface content on the following"#]
            #[doc = r#"wl_surface.commit, but this behaviour is not universal. Clients seeking to"#]
            #[doc = r#"maximise compatibility should not destroy pending buffers and should"#]
            #[doc = r#"ensure that they explicitly remove content from surfaces, even after"#]
            #[doc = r#"destroying buffers."#]
            async fn r#attach(
                client: &mut crate::Client,
                r#buffer: Option<crate::wire::ObjectId>,
                r#x: i32,
                r#y: i32,
            ) -> crate::Result<()>;
            #[doc = r#"This request is used to describe the regions where the pending"#]
            #[doc = r#"buffer is different from the current surface contents, and where"#]
            #[doc = r#"the surface therefore needs to be repainted. The compositor"#]
            #[doc = r#"ignores the parts of the damage that fall outside of the surface."#]
            #[doc = r#""#]
            #[doc = r#"Damage is double-buffered state, see wl_surface.commit."#]
            #[doc = r#""#]
            #[doc = r#"The damage rectangle is specified in surface-local coordinates,"#]
            #[doc = r#"where x and y specify the upper left corner of the damage rectangle."#]
            #[doc = r#""#]
            #[doc = r#"The initial value for pending damage is empty: no damage."#]
            #[doc = r#"wl_surface.damage adds pending damage: the new pending damage"#]
            #[doc = r#"is the union of old pending damage and the given rectangle."#]
            #[doc = r#""#]
            #[doc = r#"wl_surface.commit assigns pending damage as the current damage,"#]
            #[doc = r#"and clears pending damage. The server will clear the current"#]
            #[doc = r#"damage as it repaints the surface."#]
            #[doc = r#""#]
            #[doc = r#"Note! New clients should not use this request. Instead damage can be"#]
            #[doc = r#"posted with wl_surface.damage_buffer which uses buffer coordinates"#]
            #[doc = r#"instead of surface coordinates."#]
            async fn r#damage(
                client: &mut crate::Client,
                r#x: i32,
                r#y: i32,
                r#width: i32,
                r#height: i32,
            ) -> crate::Result<()>;
            #[doc = r#"Request a notification when it is a good time to start drawing a new"#]
            #[doc = r#"frame, by creating a frame callback. This is useful for throttling"#]
            #[doc = r#"redrawing operations, and driving animations."#]
            #[doc = r#""#]
            #[doc = r#"When a client is animating on a wl_surface, it can use the 'frame'"#]
            #[doc = r#"request to get notified when it is a good time to draw and commit the"#]
            #[doc = r#"next frame of animation. If the client commits an update earlier than"#]
            #[doc = r#"that, it is likely that some updates will not make it to the display,"#]
            #[doc = r#"and the client is wasting resources by drawing too often."#]
            #[doc = r#""#]
            #[doc = r#"The frame request will take effect on the next wl_surface.commit."#]
            #[doc = r#"The notification will only be posted for one frame unless"#]
            #[doc = r#"requested again. For a wl_surface, the notifications are posted in"#]
            #[doc = r#"the order the frame requests were committed."#]
            #[doc = r#""#]
            #[doc = r#"The server must send the notifications so that a client"#]
            #[doc = r#"will not send excessive updates, while still allowing"#]
            #[doc = r#"the highest possible update rate for clients that wait for the reply"#]
            #[doc = r#"before drawing again. The server should give some time for the client"#]
            #[doc = r#"to draw and commit after sending the frame callback events to let it"#]
            #[doc = r#"hit the next output refresh."#]
            #[doc = r#""#]
            #[doc = r#"A server should avoid signaling the frame callbacks if the"#]
            #[doc = r#"surface is not visible in any way, e.g. the surface is off-screen,"#]
            #[doc = r#"or completely obscured by other opaque surfaces."#]
            #[doc = r#""#]
            #[doc = r#"The object returned by this request will be destroyed by the"#]
            #[doc = r#"compositor after the callback is fired and as such the client must not"#]
            #[doc = r#"attempt to use it after that point."#]
            #[doc = r#""#]
            #[doc = r#"The callback_data passed in the callback is the current time, in"#]
            #[doc = r#"milliseconds, with an undefined base."#]
            async fn r#frame(
                client: &mut crate::Client,
                r#callback: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"This request sets the region of the surface that contains"#]
            #[doc = r#"opaque content."#]
            #[doc = r#""#]
            #[doc = r#"The opaque region is an optimization hint for the compositor"#]
            #[doc = r#"that lets it optimize the redrawing of content behind opaque"#]
            #[doc = r#"regions.  Setting an opaque region is not required for correct"#]
            #[doc = r#"behaviour, but marking transparent content as opaque will result"#]
            #[doc = r#"in repaint artifacts."#]
            #[doc = r#""#]
            #[doc = r#"The opaque region is specified in surface-local coordinates."#]
            #[doc = r#""#]
            #[doc = r#"The compositor ignores the parts of the opaque region that fall"#]
            #[doc = r#"outside of the surface."#]
            #[doc = r#""#]
            #[doc = r#"Opaque region is double-buffered state, see wl_surface.commit."#]
            #[doc = r#""#]
            #[doc = r#"wl_surface.set_opaque_region changes the pending opaque region."#]
            #[doc = r#"wl_surface.commit copies the pending region to the current region."#]
            #[doc = r#"Otherwise, the pending and current regions are never changed."#]
            #[doc = r#""#]
            #[doc = r#"The initial value for an opaque region is empty. Setting the pending"#]
            #[doc = r#"opaque region has copy semantics, and the wl_region object can be"#]
            #[doc = r#"destroyed immediately. A NULL wl_region causes the pending opaque"#]
            #[doc = r#"region to be set to empty."#]
            async fn r#set_opaque_region(
                client: &mut crate::Client,
                r#region: Option<crate::wire::ObjectId>,
            ) -> crate::Result<()>;
            #[doc = r#"This request sets the region of the surface that can receive"#]
            #[doc = r#"pointer and touch events."#]
            #[doc = r#""#]
            #[doc = r#"Input events happening outside of this region will try the next"#]
            #[doc = r#"surface in the server surface stack. The compositor ignores the"#]
            #[doc = r#"parts of the input region that fall outside of the surface."#]
            #[doc = r#""#]
            #[doc = r#"The input region is specified in surface-local coordinates."#]
            #[doc = r#""#]
            #[doc = r#"Input region is double-buffered state, see wl_surface.commit."#]
            #[doc = r#""#]
            #[doc = r#"wl_surface.set_input_region changes the pending input region."#]
            #[doc = r#"wl_surface.commit copies the pending region to the current region."#]
            #[doc = r#"Otherwise the pending and current regions are never changed,"#]
            #[doc = r#"except cursor and icon surfaces are special cases, see"#]
            #[doc = r#"wl_pointer.set_cursor and wl_data_device.start_drag."#]
            #[doc = r#""#]
            #[doc = r#"The initial value for an input region is infinite. That means the"#]
            #[doc = r#"whole surface will accept input. Setting the pending input region"#]
            #[doc = r#"has copy semantics, and the wl_region object can be destroyed"#]
            #[doc = r#"immediately. A NULL wl_region causes the input region to be set"#]
            #[doc = r#"to infinite."#]
            async fn r#set_input_region(
                client: &mut crate::Client,
                r#region: Option<crate::wire::ObjectId>,
            ) -> crate::Result<()>;
            #[doc = r#"Surface state (input, opaque, and damage regions, attached buffers,"#]
            #[doc = r#"etc.) is double-buffered. Protocol requests modify the pending state,"#]
            #[doc = r#"as opposed to the active state in use by the compositor."#]
            #[doc = r#""#]
            #[doc = r#"A commit request atomically creates a content update from the pending"#]
            #[doc = r#"state, even if the pending state has not been touched. The content"#]
            #[doc = r#"update is placed in a queue until it becomes active. After commit, the"#]
            #[doc = r#"new pending state is as documented for each related request."#]
            #[doc = r#""#]
            #[doc = r#"When the content update is applied, the wl_buffer is applied before all"#]
            #[doc = r#"other state. This means that all coordinates in double-buffered state"#]
            #[doc = r#"are relative to the newly attached wl_buffers, except for"#]
            #[doc = r#"wl_surface.attach itself. If there is no newly attached wl_buffer, the"#]
            #[doc = r#"coordinates are relative to the previous content update."#]
            #[doc = r#""#]
            #[doc = r#"All requests that need a commit to become effective are documented"#]
            #[doc = r#"to affect double-buffered state."#]
            #[doc = r#""#]
            #[doc = r#"Other interfaces may add further double-buffered surface state."#]
            async fn r#commit(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"This request sets the transformation that the client has already applied"#]
            #[doc = r#"to the content of the buffer. The accepted values for the transform"#]
            #[doc = r#"parameter are the values for wl_output.transform."#]
            #[doc = r#""#]
            #[doc = r#"The compositor applies the inverse of this transformation whenever it"#]
            #[doc = r#"uses the buffer contents."#]
            #[doc = r#""#]
            #[doc = r#"Buffer transform is double-buffered state, see wl_surface.commit."#]
            #[doc = r#""#]
            #[doc = r#"A newly created surface has its buffer transformation set to normal."#]
            #[doc = r#""#]
            #[doc = r#"wl_surface.set_buffer_transform changes the pending buffer"#]
            #[doc = r#"transformation. wl_surface.commit copies the pending buffer"#]
            #[doc = r#"transformation to the current one. Otherwise, the pending and current"#]
            #[doc = r#"values are never changed."#]
            #[doc = r#""#]
            #[doc = r#"The purpose of this request is to allow clients to render content"#]
            #[doc = r#"according to the output transform, thus permitting the compositor to"#]
            #[doc = r#"use certain optimizations even if the display is rotated. Using"#]
            #[doc = r#"hardware overlays and scanning out a client buffer for fullscreen"#]
            #[doc = r#"surfaces are examples of such optimizations. Those optimizations are"#]
            #[doc = r#"highly dependent on the compositor implementation, so the use of this"#]
            #[doc = r#"request should be considered on a case-by-case basis."#]
            #[doc = r#""#]
            #[doc = r#"Note that if the transform value includes 90 or 270 degree rotation,"#]
            #[doc = r#"the width of the buffer will become the surface height and the height"#]
            #[doc = r#"of the buffer will become the surface width."#]
            #[doc = r#""#]
            #[doc = r#"If transform is not one of the values from the"#]
            #[doc = r#"wl_output.transform enum the invalid_transform protocol error"#]
            #[doc = r#"is raised."#]
            async fn r#set_buffer_transform(
                client: &mut crate::Client,
                r#transform: i32,
            ) -> crate::Result<()>;
            #[doc = r#"This request sets an optional scaling factor on how the compositor"#]
            #[doc = r#"interprets the contents of the buffer attached to the window."#]
            #[doc = r#""#]
            #[doc = r#"Buffer scale is double-buffered state, see wl_surface.commit."#]
            #[doc = r#""#]
            #[doc = r#"A newly created surface has its buffer scale set to 1."#]
            #[doc = r#""#]
            #[doc = r#"wl_surface.set_buffer_scale changes the pending buffer scale."#]
            #[doc = r#"wl_surface.commit copies the pending buffer scale to the current one."#]
            #[doc = r#"Otherwise, the pending and current values are never changed."#]
            #[doc = r#""#]
            #[doc = r#"The purpose of this request is to allow clients to supply higher"#]
            #[doc = r#"resolution buffer data for use on high resolution outputs. It is"#]
            #[doc = r#"intended that you pick the same buffer scale as the scale of the"#]
            #[doc = r#"output that the surface is displayed on. This means the compositor"#]
            #[doc = r#"can avoid scaling when rendering the surface on that output."#]
            #[doc = r#""#]
            #[doc = r#"Note that if the scale is larger than 1, then you have to attach"#]
            #[doc = r#"a buffer that is larger (by a factor of scale in each dimension)"#]
            #[doc = r#"than the desired surface size."#]
            #[doc = r#""#]
            #[doc = r#"If scale is not greater than 0 the invalid_scale protocol error is"#]
            #[doc = r#"raised."#]
            async fn r#set_buffer_scale(
                client: &mut crate::Client,
                r#scale: i32,
            ) -> crate::Result<()>;
            #[doc = r#"This request is used to describe the regions where the pending"#]
            #[doc = r#"buffer is different from the current surface contents, and where"#]
            #[doc = r#"the surface therefore needs to be repainted. The compositor"#]
            #[doc = r#"ignores the parts of the damage that fall outside of the surface."#]
            #[doc = r#""#]
            #[doc = r#"Damage is double-buffered state, see wl_surface.commit."#]
            #[doc = r#""#]
            #[doc = r#"The damage rectangle is specified in buffer coordinates,"#]
            #[doc = r#"where x and y specify the upper left corner of the damage rectangle."#]
            #[doc = r#""#]
            #[doc = r#"The initial value for pending damage is empty: no damage."#]
            #[doc = r#"wl_surface.damage_buffer adds pending damage: the new pending"#]
            #[doc = r#"damage is the union of old pending damage and the given rectangle."#]
            #[doc = r#""#]
            #[doc = r#"wl_surface.commit assigns pending damage as the current damage,"#]
            #[doc = r#"and clears pending damage. The server will clear the current"#]
            #[doc = r#"damage as it repaints the surface."#]
            #[doc = r#""#]
            #[doc = r#"This request differs from wl_surface.damage in only one way - it"#]
            #[doc = r#"takes damage in buffer coordinates instead of surface-local"#]
            #[doc = r#"coordinates. While this generally is more intuitive than surface"#]
            #[doc = r#"coordinates, it is especially desirable when using wp_viewport"#]
            #[doc = r#"or when a drawing library (like EGL) is unaware of buffer scale"#]
            #[doc = r#"and buffer transform."#]
            #[doc = r#""#]
            #[doc = r#"Note: Because buffer transformation changes and damage requests may"#]
            #[doc = r#"be interleaved in the protocol stream, it is impossible to determine"#]
            #[doc = r#"the actual mapping between surface and buffer damage until"#]
            #[doc = r#"wl_surface.commit time. Therefore, compositors wishing to take both"#]
            #[doc = r#"kinds of damage into account will have to accumulate damage from the"#]
            #[doc = r#"two requests separately and only transform from one to the other"#]
            #[doc = r#"after receiving the wl_surface.commit."#]
            async fn r#damage_buffer(
                client: &mut crate::Client,
                r#x: i32,
                r#y: i32,
                r#width: i32,
                r#height: i32,
            ) -> crate::Result<()>;
            #[doc = r#"The x and y arguments specify the location of the new pending"#]
            #[doc = r#"buffer's upper left corner, relative to the current buffer's upper"#]
            #[doc = r#"left corner, in surface-local coordinates. In other words, the"#]
            #[doc = r#"x and y, combined with the new surface size define in which"#]
            #[doc = r#"directions the surface's size changes."#]
            #[doc = r#""#]
            #[doc = r#"Surface location offset is double-buffered state, see"#]
            #[doc = r#"wl_surface.commit."#]
            #[doc = r#""#]
            #[doc = r#"This request is semantically equivalent to and the replaces the x and y"#]
            #[doc = r#"arguments in the wl_surface.attach request in wl_surface versions prior"#]
            #[doc = r#"to 5. See wl_surface.attach for details."#]
            async fn r#offset(client: &mut crate::Client, r#x: i32, r#y: i32) -> crate::Result<()>;
            #[doc = r#"This is emitted whenever a surface's creation, movement, or resizing"#]
            #[doc = r#"results in some part of it being within the scanout region of an"#]
            #[doc = r#"output."#]
            #[doc = r#""#]
            #[doc = r#"Note that a surface may be overlapping with zero or more outputs."#]
            async fn r#enter(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#output: crate::wire::ObjectId,
            ) -> crate::Result<()> {
                tracing::debug!("wl_surface -> enter");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_object(Some(output))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This is emitted whenever a surface's creation, movement, or resizing"#]
            #[doc = r#"results in it no longer having any part of it within the scanout region"#]
            #[doc = r#"of an output."#]
            #[doc = r#""#]
            #[doc = r#"Clients should not use the number of outputs the surface is on for frame"#]
            #[doc = r#"throttling purposes. The surface might be hidden even if no leave event"#]
            #[doc = r#"has been sent, and the compositor might expect new surface content"#]
            #[doc = r#"updates even if no enter event has been sent. The frame event should be"#]
            #[doc = r#"used instead."#]
            async fn r#leave(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#output: crate::wire::ObjectId,
            ) -> crate::Result<()> {
                tracing::debug!("wl_surface -> leave");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_object(Some(output))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event indicates the preferred buffer scale for this surface. It is"#]
            #[doc = r#"sent whenever the compositor's preference changes."#]
            #[doc = r#""#]
            #[doc = r#"Before receiving this event the preferred buffer scale for this surface"#]
            #[doc = r#"is 1."#]
            #[doc = r#""#]
            #[doc = r#"It is intended that scaling aware clients use this event to scale their"#]
            #[doc = r#"content and use wl_surface.set_buffer_scale to indicate the scale they"#]
            #[doc = r#"have rendered with. This allows clients to supply a higher detail"#]
            #[doc = r#"buffer."#]
            #[doc = r#""#]
            #[doc = r#"The compositor shall emit a scale value greater than 0."#]
            async fn r#preferred_buffer_scale(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#factor: i32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_surface -> preferred_buffer_scale");
                let payload = crate::wire::PayloadBuilder::new().put_int(factor).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event indicates the preferred buffer transform for this surface."#]
            #[doc = r#"It is sent whenever the compositor's preference changes."#]
            #[doc = r#""#]
            #[doc = r#"Before receiving this event the preferred buffer transform for this"#]
            #[doc = r#"surface is normal."#]
            #[doc = r#""#]
            #[doc = r#"Applying this transformation to the surface buffer contents and using"#]
            #[doc = r#"wl_surface.set_buffer_transform might allow the compositor to use the"#]
            #[doc = r#"surface buffer more efficiently."#]
            async fn r#preferred_buffer_transform(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#transform: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_surface -> preferred_buffer_transform");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(transform)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 3, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod wl_seat {
        enum r#Capability {}
        enum r#Error {}
        #[doc = r#"A seat is a group of keyboards, pointer and touch devices. This"#]
        #[doc = r#"object is published as a global during start up, or when such a"#]
        #[doc = r#"device is hot plugged.  A seat typically has a pointer and"#]
        #[doc = r#"maintains a keyboard focus and a pointer focus."#]
        pub trait r#WlSeat {
            const INTERFACE: &'static str = "wl_seat";
            const VERSION: u32 = 9;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_seat -> get_pointer");
                        Self::r#get_pointer(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1 => {
                        tracing::debug!("wl_seat -> get_keyboard");
                        Self::r#get_keyboard(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2 => {
                        tracing::debug!("wl_seat -> get_touch");
                        Self::r#get_touch(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    3 => {
                        tracing::debug!("wl_seat -> release");
                        Self::r#release(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"The ID provided will be initialized to the wl_pointer interface"#]
            #[doc = r#"for this seat."#]
            #[doc = r#""#]
            #[doc = r#"This request only takes effect if the seat has the pointer"#]
            #[doc = r#"capability, or has had the pointer capability in the past."#]
            #[doc = r#"It is a protocol violation to issue this request on a seat that has"#]
            #[doc = r#"never had the pointer capability. The missing_capability error will"#]
            #[doc = r#"be sent in this case."#]
            async fn r#get_pointer(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"The ID provided will be initialized to the wl_keyboard interface"#]
            #[doc = r#"for this seat."#]
            #[doc = r#""#]
            #[doc = r#"This request only takes effect if the seat has the keyboard"#]
            #[doc = r#"capability, or has had the keyboard capability in the past."#]
            #[doc = r#"It is a protocol violation to issue this request on a seat that has"#]
            #[doc = r#"never had the keyboard capability. The missing_capability error will"#]
            #[doc = r#"be sent in this case."#]
            async fn r#get_keyboard(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"The ID provided will be initialized to the wl_touch interface"#]
            #[doc = r#"for this seat."#]
            #[doc = r#""#]
            #[doc = r#"This request only takes effect if the seat has the touch"#]
            #[doc = r#"capability, or has had the touch capability in the past."#]
            #[doc = r#"It is a protocol violation to issue this request on a seat that has"#]
            #[doc = r#"never had the touch capability. The missing_capability error will"#]
            #[doc = r#"be sent in this case."#]
            async fn r#get_touch(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"Using this request a client can tell the server that it is not going to"#]
            #[doc = r#"use the seat object anymore."#]
            async fn r#release(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"This is emitted whenever a seat gains or loses the pointer,"#]
            #[doc = r#"keyboard or touch capabilities.  The argument is a capability"#]
            #[doc = r#"enum containing the complete set of capabilities this seat has."#]
            #[doc = r#""#]
            #[doc = r#"When the pointer capability is added, a client may create a"#]
            #[doc = r#"wl_pointer object using the wl_seat.get_pointer request. This object"#]
            #[doc = r#"will receive pointer events until the capability is removed in the"#]
            #[doc = r#"future."#]
            #[doc = r#""#]
            #[doc = r#"When the pointer capability is removed, a client should destroy the"#]
            #[doc = r#"wl_pointer objects associated with the seat where the capability was"#]
            #[doc = r#"removed, using the wl_pointer.release request. No further pointer"#]
            #[doc = r#"events will be received on these objects."#]
            #[doc = r#""#]
            #[doc = r#"In some compositors, if a seat regains the pointer capability and a"#]
            #[doc = r#"client has a previously obtained wl_pointer object of version 4 or"#]
            #[doc = r#"less, that object may start sending pointer events again. This"#]
            #[doc = r#"behavior is considered a misinterpretation of the intended behavior"#]
            #[doc = r#"and must not be relied upon by the client. wl_pointer objects of"#]
            #[doc = r#"version 5 or later must not send events if created before the most"#]
            #[doc = r#"recent event notifying the client of an added pointer capability."#]
            #[doc = r#""#]
            #[doc = r#"The above behavior also applies to wl_keyboard and wl_touch with the"#]
            #[doc = r#"keyboard and touch capabilities, respectively."#]
            async fn r#capabilities(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#capabilities: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_seat -> capabilities");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(capabilities)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"In a multi-seat configuration the seat name can be used by clients to"#]
            #[doc = r#"help identify which physical devices the seat represents."#]
            #[doc = r#""#]
            #[doc = r#"The seat name is a UTF-8 string with no convention defined for its"#]
            #[doc = r#"contents. Each name is unique among all wl_seat globals. The name is"#]
            #[doc = r#"only guaranteed to be unique for the current compositor instance."#]
            #[doc = r#""#]
            #[doc = r#"The same seat names are used for all clients. Thus, the name can be"#]
            #[doc = r#"shared across processes to refer to a specific wl_seat global."#]
            #[doc = r#""#]
            #[doc = r#"The name event is sent after binding to the seat global. This event is"#]
            #[doc = r#"only sent once per seat object, and the name does not change over the"#]
            #[doc = r#"lifetime of the wl_seat global."#]
            #[doc = r#""#]
            #[doc = r#"Compositors may re-use the same seat name if the wl_seat global is"#]
            #[doc = r#"destroyed and re-created later."#]
            async fn r#name(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#name: String,
            ) -> crate::Result<()> {
                tracing::debug!("wl_seat -> name");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_string(Some(name))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod wl_pointer {
        enum r#Error {}
        enum r#ButtonState {}
        enum r#Axis {}
        enum r#AxisSource {}
        enum r#AxisRelativeDirection {}
        #[doc = r#"The wl_pointer interface represents one or more input devices,"#]
        #[doc = r#"such as mice, which control the pointer location and pointer_focus"#]
        #[doc = r#"of a seat."#]
        #[doc = r#""#]
        #[doc = r#"The wl_pointer interface generates motion, enter and leave"#]
        #[doc = r#"events for the surfaces that the pointer is located over,"#]
        #[doc = r#"and button and axis events for button presses, button releases"#]
        #[doc = r#"and scrolling."#]
        pub trait r#WlPointer {
            const INTERFACE: &'static str = "wl_pointer";
            const VERSION: u32 = 9;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_pointer -> set_cursor");
                        Self::r#set_cursor(
                            client,
                            message.uint()?,
                            message.object()?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    1 => {
                        tracing::debug!("wl_pointer -> release");
                        Self::r#release(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Set the pointer surface, i.e., the surface that contains the"#]
            #[doc = r#"pointer image (cursor). This request gives the surface the role"#]
            #[doc = r#"of a cursor. If the surface already has another role, it raises"#]
            #[doc = r#"a protocol error."#]
            #[doc = r#""#]
            #[doc = r#"The cursor actually changes only if the pointer"#]
            #[doc = r#"focus for this device is one of the requesting client's surfaces"#]
            #[doc = r#"or the surface parameter is the current pointer surface. If"#]
            #[doc = r#"there was a previous surface set with this request it is"#]
            #[doc = r#"replaced. If surface is NULL, the pointer image is hidden."#]
            #[doc = r#""#]
            #[doc = r#"The parameters hotspot_x and hotspot_y define the position of"#]
            #[doc = r#"the pointer surface relative to the pointer location. Its"#]
            #[doc = r#"top-left corner is always at (x, y) - (hotspot_x, hotspot_y),"#]
            #[doc = r#"where (x, y) are the coordinates of the pointer location, in"#]
            #[doc = r#"surface-local coordinates."#]
            #[doc = r#""#]
            #[doc = r#"On wl_surface.offset requests to the pointer surface, hotspot_x"#]
            #[doc = r#"and hotspot_y are decremented by the x and y parameters"#]
            #[doc = r#"passed to the request. The offset must be applied by"#]
            #[doc = r#"wl_surface.commit as usual."#]
            #[doc = r#""#]
            #[doc = r#"The hotspot can also be updated by passing the currently set"#]
            #[doc = r#"pointer surface to this request with new values for hotspot_x"#]
            #[doc = r#"and hotspot_y."#]
            #[doc = r#""#]
            #[doc = r#"The input region is ignored for wl_surfaces with the role of"#]
            #[doc = r#"a cursor. When the use as a cursor ends, the wl_surface is"#]
            #[doc = r#"unmapped."#]
            #[doc = r#""#]
            #[doc = r#"The serial parameter must match the latest wl_pointer.enter"#]
            #[doc = r#"serial number sent to the client. Otherwise the request will be"#]
            #[doc = r#"ignored."#]
            async fn r#set_cursor(
                client: &mut crate::Client,
                r#serial: u32,
                r#surface: Option<crate::wire::ObjectId>,
                r#hotspot_x: i32,
                r#hotspot_y: i32,
            ) -> crate::Result<()>;
            #[doc = r#"Using this request a client can tell the server that it is not going to"#]
            #[doc = r#"use the pointer object anymore."#]
            #[doc = r#""#]
            #[doc = r#"This request destroys the pointer proxy object, so clients must not call"#]
            #[doc = r#"wl_pointer_destroy() after using this request."#]
            async fn r#release(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Notification that this seat's pointer is focused on a certain"#]
            #[doc = r#"surface."#]
            #[doc = r#""#]
            #[doc = r#"When a seat's focus enters a surface, the pointer image"#]
            #[doc = r#"is undefined and a client should respond to this event by setting"#]
            #[doc = r#"an appropriate pointer image with the set_cursor request."#]
            async fn r#enter(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
                r#surface: crate::wire::ObjectId,
                r#surface_x: crate::wire::Fixed,
                r#surface_y: crate::wire::Fixed,
            ) -> crate::Result<()> {
                tracing::debug!("wl_pointer -> enter");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(Some(surface))
                    .put_fixed(surface_x)
                    .put_fixed(surface_y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Notification that this seat's pointer is no longer focused on"#]
            #[doc = r#"a certain surface."#]
            #[doc = r#""#]
            #[doc = r#"The leave notification is sent before the enter notification"#]
            #[doc = r#"for the new focus."#]
            async fn r#leave(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
                r#surface: crate::wire::ObjectId,
            ) -> crate::Result<()> {
                tracing::debug!("wl_pointer -> leave");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Notification of pointer location change. The arguments"#]
            #[doc = r#"surface_x and surface_y are the location relative to the"#]
            #[doc = r#"focused surface."#]
            async fn r#motion(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#time: u32,
                r#surface_x: crate::wire::Fixed,
                r#surface_y: crate::wire::Fixed,
            ) -> crate::Result<()> {
                tracing::debug!("wl_pointer -> motion");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_fixed(surface_x)
                    .put_fixed(surface_y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Mouse button click and release notifications."#]
            #[doc = r#""#]
            #[doc = r#"The location of the click is given by the last motion or"#]
            #[doc = r#"enter event."#]
            #[doc = r#"The time argument is a timestamp with millisecond"#]
            #[doc = r#"granularity, with an undefined base."#]
            #[doc = r#""#]
            #[doc = r#"The button is a button code as defined in the Linux kernel's"#]
            #[doc = r#"linux/input-event-codes.h header file, e.g. BTN_LEFT."#]
            #[doc = r#""#]
            #[doc = r#"Any 16-bit button code value is reserved for future additions to the"#]
            #[doc = r#"kernel's event code list. All other button codes above 0xFFFF are"#]
            #[doc = r#"currently undefined but may be used in future versions of this"#]
            #[doc = r#"protocol."#]
            async fn r#button(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
                r#time: u32,
                r#button: u32,
                r#state: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_pointer -> button");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(time)
                    .put_uint(button)
                    .put_uint(state)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 3, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Scroll and other axis notifications."#]
            #[doc = r#""#]
            #[doc = r#"For scroll events (vertical and horizontal scroll axes), the"#]
            #[doc = r#"value parameter is the length of a vector along the specified"#]
            #[doc = r#"axis in a coordinate space identical to those of motion events,"#]
            #[doc = r#"representing a relative movement along the specified axis."#]
            #[doc = r#""#]
            #[doc = r#"For devices that support movements non-parallel to axes multiple"#]
            #[doc = r#"axis events will be emitted."#]
            #[doc = r#""#]
            #[doc = r#"When applicable, for example for touch pads, the server can"#]
            #[doc = r#"choose to emit scroll events where the motion vector is"#]
            #[doc = r#"equivalent to a motion event vector."#]
            #[doc = r#""#]
            #[doc = r#"When applicable, a client can transform its content relative to the"#]
            #[doc = r#"scroll distance."#]
            async fn r#axis(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#time: u32,
                r#axis: u32,
                r#value: crate::wire::Fixed,
            ) -> crate::Result<()> {
                tracing::debug!("wl_pointer -> axis");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_uint(axis)
                    .put_fixed(value)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 4, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Indicates the end of a set of events that logically belong together."#]
            #[doc = r#"A client is expected to accumulate the data in all events within the"#]
            #[doc = r#"frame before proceeding."#]
            #[doc = r#""#]
            #[doc = r#"All wl_pointer events before a wl_pointer.frame event belong"#]
            #[doc = r#"logically together. For example, in a diagonal scroll motion the"#]
            #[doc = r#"compositor will send an optional wl_pointer.axis_source event, two"#]
            #[doc = r#"wl_pointer.axis events (horizontal and vertical) and finally a"#]
            #[doc = r#"wl_pointer.frame event. The client may use this information to"#]
            #[doc = r#"calculate a diagonal vector for scrolling."#]
            #[doc = r#""#]
            #[doc = r#"When multiple wl_pointer.axis events occur within the same frame,"#]
            #[doc = r#"the motion vector is the combined motion of all events."#]
            #[doc = r#"When a wl_pointer.axis and a wl_pointer.axis_stop event occur within"#]
            #[doc = r#"the same frame, this indicates that axis movement in one axis has"#]
            #[doc = r#"stopped but continues in the other axis."#]
            #[doc = r#"When multiple wl_pointer.axis_stop events occur within the same"#]
            #[doc = r#"frame, this indicates that these axes stopped in the same instance."#]
            #[doc = r#""#]
            #[doc = r#"A wl_pointer.frame event is sent for every logical event group,"#]
            #[doc = r#"even if the group only contains a single wl_pointer event."#]
            #[doc = r#"Specifically, a client may get a sequence: motion, frame, button,"#]
            #[doc = r#"frame, axis, frame, axis_stop, frame."#]
            #[doc = r#""#]
            #[doc = r#"The wl_pointer.enter and wl_pointer.leave events are logical events"#]
            #[doc = r#"generated by the compositor and not the hardware. These events are"#]
            #[doc = r#"also grouped by a wl_pointer.frame. When a pointer moves from one"#]
            #[doc = r#"surface to another, a compositor should group the"#]
            #[doc = r#"wl_pointer.leave event within the same wl_pointer.frame."#]
            #[doc = r#"However, a client must not rely on wl_pointer.leave and"#]
            #[doc = r#"wl_pointer.enter being in the same wl_pointer.frame."#]
            #[doc = r#"Compositor-specific policies may require the wl_pointer.leave and"#]
            #[doc = r#"wl_pointer.enter event being split across multiple wl_pointer.frame"#]
            #[doc = r#"groups."#]
            async fn r#frame(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("wl_pointer -> frame");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 5, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Source information for scroll and other axes."#]
            #[doc = r#""#]
            #[doc = r#"This event does not occur on its own. It is sent before a"#]
            #[doc = r#"wl_pointer.frame event and carries the source information for"#]
            #[doc = r#"all events within that frame."#]
            #[doc = r#""#]
            #[doc = r#"The source specifies how this event was generated. If the source is"#]
            #[doc = r#"wl_pointer.axis_source.finger, a wl_pointer.axis_stop event will be"#]
            #[doc = r#"sent when the user lifts the finger off the device."#]
            #[doc = r#""#]
            #[doc = r#"If the source is wl_pointer.axis_source.wheel,"#]
            #[doc = r#"wl_pointer.axis_source.wheel_tilt or"#]
            #[doc = r#"wl_pointer.axis_source.continuous, a wl_pointer.axis_stop event may"#]
            #[doc = r#"or may not be sent. Whether a compositor sends an axis_stop event"#]
            #[doc = r#"for these sources is hardware-specific and implementation-dependent;"#]
            #[doc = r#"clients must not rely on receiving an axis_stop event for these"#]
            #[doc = r#"scroll sources and should treat scroll sequences from these scroll"#]
            #[doc = r#"sources as unterminated by default."#]
            #[doc = r#""#]
            #[doc = r#"This event is optional. If the source is unknown for a particular"#]
            #[doc = r#"axis event sequence, no event is sent."#]
            #[doc = r#"Only one wl_pointer.axis_source event is permitted per frame."#]
            #[doc = r#""#]
            #[doc = r#"The order of wl_pointer.axis_discrete and wl_pointer.axis_source is"#]
            #[doc = r#"not guaranteed."#]
            async fn r#axis_source(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#axis_source: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_pointer -> axis_source");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(axis_source)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 6, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Stop notification for scroll and other axes."#]
            #[doc = r#""#]
            #[doc = r#"For some wl_pointer.axis_source types, a wl_pointer.axis_stop event"#]
            #[doc = r#"is sent to notify a client that the axis sequence has terminated."#]
            #[doc = r#"This enables the client to implement kinetic scrolling."#]
            #[doc = r#"See the wl_pointer.axis_source documentation for information on when"#]
            #[doc = r#"this event may be generated."#]
            #[doc = r#""#]
            #[doc = r#"Any wl_pointer.axis events with the same axis_source after this"#]
            #[doc = r#"event should be considered as the start of a new axis motion."#]
            #[doc = r#""#]
            #[doc = r#"The timestamp is to be interpreted identical to the timestamp in the"#]
            #[doc = r#"wl_pointer.axis event. The timestamp value may be the same as a"#]
            #[doc = r#"preceding wl_pointer.axis event."#]
            async fn r#axis_stop(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#time: u32,
                r#axis: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_pointer -> axis_stop");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_uint(axis)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 7, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Discrete step information for scroll and other axes."#]
            #[doc = r#""#]
            #[doc = r#"This event carries the axis value of the wl_pointer.axis event in"#]
            #[doc = r#"discrete steps (e.g. mouse wheel clicks)."#]
            #[doc = r#""#]
            #[doc = r#"This event is deprecated with wl_pointer version 8 - this event is not"#]
            #[doc = r#"sent to clients supporting version 8 or later."#]
            #[doc = r#""#]
            #[doc = r#"This event does not occur on its own, it is coupled with a"#]
            #[doc = r#"wl_pointer.axis event that represents this axis value on a"#]
            #[doc = r#"continuous scale. The protocol guarantees that each axis_discrete"#]
            #[doc = r#"event is always followed by exactly one axis event with the same"#]
            #[doc = r#"axis number within the same wl_pointer.frame. Note that the protocol"#]
            #[doc = r#"allows for other events to occur between the axis_discrete and"#]
            #[doc = r#"its coupled axis event, including other axis_discrete or axis"#]
            #[doc = r#"events. A wl_pointer.frame must not contain more than one axis_discrete"#]
            #[doc = r#"event per axis type."#]
            #[doc = r#""#]
            #[doc = r#"This event is optional; continuous scrolling devices"#]
            #[doc = r#"like two-finger scrolling on touchpads do not have discrete"#]
            #[doc = r#"steps and do not generate this event."#]
            #[doc = r#""#]
            #[doc = r#"The discrete value carries the directional information. e.g. a value"#]
            #[doc = r#"of -2 is two steps towards the negative direction of this axis."#]
            #[doc = r#""#]
            #[doc = r#"The axis number is identical to the axis number in the associated"#]
            #[doc = r#"axis event."#]
            #[doc = r#""#]
            #[doc = r#"The order of wl_pointer.axis_discrete and wl_pointer.axis_source is"#]
            #[doc = r#"not guaranteed."#]
            async fn r#axis_discrete(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#axis: u32,
                r#discrete: i32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_pointer -> axis_discrete");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(axis)
                    .put_int(discrete)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 8, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Discrete high-resolution scroll information."#]
            #[doc = r#""#]
            #[doc = r#"This event carries high-resolution wheel scroll information,"#]
            #[doc = r#"with each multiple of 120 representing one logical scroll step"#]
            #[doc = r#"(a wheel detent). For example, an axis_value120 of 30 is one quarter of"#]
            #[doc = r#"a logical scroll step in the positive direction, a value120 of"#]
            #[doc = r#"-240 are two logical scroll steps in the negative direction within the"#]
            #[doc = r#"same hardware event."#]
            #[doc = r#"Clients that rely on discrete scrolling should accumulate the"#]
            #[doc = r#"value120 to multiples of 120 before processing the event."#]
            #[doc = r#""#]
            #[doc = r#"The value120 must not be zero."#]
            #[doc = r#""#]
            #[doc = r#"This event replaces the wl_pointer.axis_discrete event in clients"#]
            #[doc = r#"supporting wl_pointer version 8 or later."#]
            #[doc = r#""#]
            #[doc = r#"Where a wl_pointer.axis_source event occurs in the same"#]
            #[doc = r#"wl_pointer.frame, the axis source applies to this event."#]
            #[doc = r#""#]
            #[doc = r#"The order of wl_pointer.axis_value120 and wl_pointer.axis_source is"#]
            #[doc = r#"not guaranteed."#]
            async fn r#axis_value120(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#axis: u32,
                r#value120: i32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_pointer -> axis_value120");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(axis)
                    .put_int(value120)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 9, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Relative directional information of the entity causing the axis"#]
            #[doc = r#"motion."#]
            #[doc = r#""#]
            #[doc = r#"For a wl_pointer.axis event, the wl_pointer.axis_relative_direction"#]
            #[doc = r#"event specifies the movement direction of the entity causing the"#]
            #[doc = r#"wl_pointer.axis event. For example:"#]
            #[doc = r#"- if a user's fingers on a touchpad move down and this"#]
            #[doc = r#"causes a wl_pointer.axis vertical_scroll down event, the physical"#]
            #[doc = r#"direction is 'identical'"#]
            #[doc = r#"- if a user's fingers on a touchpad move down and this causes a"#]
            #[doc = r#"wl_pointer.axis vertical_scroll up scroll up event ('natural"#]
            #[doc = r#"scrolling'), the physical direction is 'inverted'."#]
            #[doc = r#""#]
            #[doc = r#"A client may use this information to adjust scroll motion of"#]
            #[doc = r#"components. Specifically, enabling natural scrolling causes the"#]
            #[doc = r#"content to change direction compared to traditional scrolling."#]
            #[doc = r#"Some widgets like volume control sliders should usually match the"#]
            #[doc = r#"physical direction regardless of whether natural scrolling is"#]
            #[doc = r#"active. This event enables clients to match the scroll direction of"#]
            #[doc = r#"a widget to the physical direction."#]
            #[doc = r#""#]
            #[doc = r#"This event does not occur on its own, it is coupled with a"#]
            #[doc = r#"wl_pointer.axis event that represents this axis value."#]
            #[doc = r#"The protocol guarantees that each axis_relative_direction event is"#]
            #[doc = r#"always followed by exactly one axis event with the same"#]
            #[doc = r#"axis number within the same wl_pointer.frame. Note that the protocol"#]
            #[doc = r#"allows for other events to occur between the axis_relative_direction"#]
            #[doc = r#"and its coupled axis event."#]
            #[doc = r#""#]
            #[doc = r#"The axis number is identical to the axis number in the associated"#]
            #[doc = r#"axis event."#]
            #[doc = r#""#]
            #[doc = r#"The order of wl_pointer.axis_relative_direction,"#]
            #[doc = r#"wl_pointer.axis_discrete and wl_pointer.axis_source is not"#]
            #[doc = r#"guaranteed."#]
            async fn r#axis_relative_direction(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#axis: u32,
                r#direction: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_pointer -> axis_relative_direction");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(axis)
                    .put_uint(direction)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 10, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod wl_keyboard {
        enum r#KeymapFormat {}
        enum r#KeyState {}
        #[doc = r#"The wl_keyboard interface represents one or more keyboards"#]
        #[doc = r#"associated with a seat."#]
        #[doc = r#""#]
        #[doc = r#"Each wl_keyboard has the following logical state:"#]
        #[doc = r#""#]
        #[doc = r#"- an active surface (possibly null),"#]
        #[doc = r#"- the keys currently logically down,"#]
        #[doc = r#"- the active modifiers,"#]
        #[doc = r#"- the active group."#]
        #[doc = r#""#]
        #[doc = r#"By default, the active surface is null, the keys currently logically down"#]
        #[doc = r#"are empty, the active modifiers and the active group are 0."#]
        pub trait r#WlKeyboard {
            const INTERFACE: &'static str = "wl_keyboard";
            const VERSION: u32 = 9;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_keyboard -> release");
                        Self::r#release(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            async fn r#release(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"This event provides a file descriptor to the client which can be"#]
            #[doc = r#"memory-mapped in read-only mode to provide a keyboard mapping"#]
            #[doc = r#"description."#]
            #[doc = r#""#]
            #[doc = r#"From version 7 onwards, the fd must be mapped with MAP_PRIVATE by"#]
            #[doc = r#"the recipient, as MAP_SHARED may fail."#]
            async fn r#keymap(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#format: u32,
                r#fd: std::os::fd::RawFd,
                r#size: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_keyboard -> keymap");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(format)
                    .put_int(fd)
                    .put_uint(size)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Notification that this seat's keyboard focus is on a certain"#]
            #[doc = r#"surface."#]
            #[doc = r#""#]
            #[doc = r#"The compositor must send the wl_keyboard.modifiers event after this"#]
            #[doc = r#"event."#]
            #[doc = r#""#]
            #[doc = r#"In the wl_keyboard logical state, this event sets the active surface to"#]
            #[doc = r#"the surface argument and the keys currently logically down to the keys"#]
            #[doc = r#"in the keys argument. The compositor must not send this event if the"#]
            #[doc = r#"wl_keyboard already had an active surface immediately before this event."#]
            async fn r#enter(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
                r#surface: crate::wire::ObjectId,
                r#keys: Vec<u8>,
            ) -> crate::Result<()> {
                tracing::debug!("wl_keyboard -> enter");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(Some(surface))
                    .put_array(keys)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Notification that this seat's keyboard focus is no longer on"#]
            #[doc = r#"a certain surface."#]
            #[doc = r#""#]
            #[doc = r#"The leave notification is sent before the enter notification"#]
            #[doc = r#"for the new focus."#]
            #[doc = r#""#]
            #[doc = r#"In the wl_keyboard logical state, this event resets all values to their"#]
            #[doc = r#"defaults. The compositor must not send this event if the active surface"#]
            #[doc = r#"of the wl_keyboard was not equal to the surface argument immediately"#]
            #[doc = r#"before this event."#]
            async fn r#leave(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
                r#surface: crate::wire::ObjectId,
            ) -> crate::Result<()> {
                tracing::debug!("wl_keyboard -> leave");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"A key was pressed or released."#]
            #[doc = r#"The time argument is a timestamp with millisecond"#]
            #[doc = r#"granularity, with an undefined base."#]
            #[doc = r#""#]
            #[doc = r#"The key is a platform-specific key code that can be interpreted"#]
            #[doc = r#"by feeding it to the keyboard mapping (see the keymap event)."#]
            #[doc = r#""#]
            #[doc = r#"If this event produces a change in modifiers, then the resulting"#]
            #[doc = r#"wl_keyboard.modifiers event must be sent after this event."#]
            #[doc = r#""#]
            #[doc = r#"In the wl_keyboard logical state, this event adds the key to the keys"#]
            #[doc = r#"currently logically down (if the state argument is pressed) or removes"#]
            #[doc = r#"the key from the keys currently logically down (if the state argument is"#]
            #[doc = r#"released). The compositor must not send this event if the wl_keyboard"#]
            #[doc = r#"did not have an active surface immediately before this event. The"#]
            #[doc = r#"compositor must not send this event if state is pressed (resp. released)"#]
            #[doc = r#"and the key was already logically down (resp. was not logically down)"#]
            #[doc = r#"immediately before this event."#]
            async fn r#key(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
                r#time: u32,
                r#key: u32,
                r#state: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_keyboard -> key");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(time)
                    .put_uint(key)
                    .put_uint(state)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 3, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Notifies clients that the modifier and/or group state has"#]
            #[doc = r#"changed, and it should update its local state."#]
            #[doc = r#""#]
            #[doc = r#"The compositor may send this event without a surface of the client"#]
            #[doc = r#"having keyboard focus, for example to tie modifier information to"#]
            #[doc = r#"pointer focus instead. If a modifier event with pressed modifiers is sent"#]
            #[doc = r#"without a prior enter event, the client can assume the modifier state is"#]
            #[doc = r#"valid until it receives the next wl_keyboard.modifiers event. In order to"#]
            #[doc = r#"reset the modifier state again, the compositor can send a"#]
            #[doc = r#"wl_keyboard.modifiers event with no pressed modifiers."#]
            #[doc = r#""#]
            #[doc = r#"In the wl_keyboard logical state, this event updates the modifiers and"#]
            #[doc = r#"group."#]
            async fn r#modifiers(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
                r#mods_depressed: u32,
                r#mods_latched: u32,
                r#mods_locked: u32,
                r#group: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_keyboard -> modifiers");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(mods_depressed)
                    .put_uint(mods_latched)
                    .put_uint(mods_locked)
                    .put_uint(group)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 4, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Informs the client about the keyboard's repeat rate and delay."#]
            #[doc = r#""#]
            #[doc = r#"This event is sent as soon as the wl_keyboard object has been created,"#]
            #[doc = r#"and is guaranteed to be received by the client before any key press"#]
            #[doc = r#"event."#]
            #[doc = r#""#]
            #[doc = r#"Negative values for either rate or delay are illegal. A rate of zero"#]
            #[doc = r#"will disable any repeating (regardless of the value of delay)."#]
            #[doc = r#""#]
            #[doc = r#"This event can be sent later on as well with a new value if necessary,"#]
            #[doc = r#"so clients should continue listening for the event past the creation"#]
            #[doc = r#"of wl_keyboard."#]
            async fn r#repeat_info(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#rate: i32,
                r#delay: i32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_keyboard -> repeat_info");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_int(rate)
                    .put_int(delay)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 5, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod wl_touch {
        #[doc = r#"The wl_touch interface represents a touchscreen"#]
        #[doc = r#"associated with a seat."#]
        #[doc = r#""#]
        #[doc = r#"Touch interactions can consist of one or more contacts."#]
        #[doc = r#"For each contact, a series of events is generated, starting"#]
        #[doc = r#"with a down event, followed by zero or more motion events,"#]
        #[doc = r#"and ending with an up event. Events relating to the same"#]
        #[doc = r#"contact point can be identified by the ID of the sequence."#]
        pub trait r#WlTouch {
            const INTERFACE: &'static str = "wl_touch";
            const VERSION: u32 = 9;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_touch -> release");
                        Self::r#release(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            async fn r#release(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"A new touch point has appeared on the surface. This touch point is"#]
            #[doc = r#"assigned a unique ID. Future events from this touch point reference"#]
            #[doc = r#"this ID. The ID ceases to be valid after a touch up event and may be"#]
            #[doc = r#"reused in the future."#]
            async fn r#down(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
                r#time: u32,
                r#surface: crate::wire::ObjectId,
                r#id: i32,
                r#x: crate::wire::Fixed,
                r#y: crate::wire::Fixed,
            ) -> crate::Result<()> {
                tracing::debug!("wl_touch -> down");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(time)
                    .put_object(Some(surface))
                    .put_int(id)
                    .put_fixed(x)
                    .put_fixed(y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"The touch point has disappeared. No further events will be sent for"#]
            #[doc = r#"this touch point and the touch point's ID is released and may be"#]
            #[doc = r#"reused in a future touch down event."#]
            async fn r#up(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
                r#time: u32,
                r#id: i32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_touch -> up");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(time)
                    .put_int(id)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"A touch point has changed coordinates."#]
            async fn r#motion(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#time: u32,
                r#id: i32,
                r#x: crate::wire::Fixed,
                r#y: crate::wire::Fixed,
            ) -> crate::Result<()> {
                tracing::debug!("wl_touch -> motion");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_int(id)
                    .put_fixed(x)
                    .put_fixed(y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Indicates the end of a set of events that logically belong together."#]
            #[doc = r#"A client is expected to accumulate the data in all events within the"#]
            #[doc = r#"frame before proceeding."#]
            #[doc = r#""#]
            #[doc = r#"A wl_touch.frame terminates at least one event but otherwise no"#]
            #[doc = r#"guarantee is provided about the set of events within a frame. A client"#]
            #[doc = r#"must assume that any state not updated in a frame is unchanged from the"#]
            #[doc = r#"previously known state."#]
            async fn r#frame(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("wl_touch -> frame");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 3, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent if the compositor decides the touch stream is a global"#]
            #[doc = r#"gesture. No further events are sent to the clients from that"#]
            #[doc = r#"particular gesture. Touch cancellation applies to all touch points"#]
            #[doc = r#"currently active on this client's surface. The client is"#]
            #[doc = r#"responsible for finalizing the touch points, future touch points on"#]
            #[doc = r#"this surface may reuse the touch point ID."#]
            #[doc = r#""#]
            #[doc = r#"No frame event is required after the cancel event."#]
            async fn r#cancel(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("wl_touch -> cancel");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 4, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent when a touchpoint has changed its shape."#]
            #[doc = r#""#]
            #[doc = r#"This event does not occur on its own. It is sent before a"#]
            #[doc = r#"wl_touch.frame event and carries the new shape information for"#]
            #[doc = r#"any previously reported, or new touch points of that frame."#]
            #[doc = r#""#]
            #[doc = r#"Other events describing the touch point such as wl_touch.down,"#]
            #[doc = r#"wl_touch.motion or wl_touch.orientation may be sent within the"#]
            #[doc = r#"same wl_touch.frame. A client should treat these events as a single"#]
            #[doc = r#"logical touch point update. The order of wl_touch.shape,"#]
            #[doc = r#"wl_touch.orientation and wl_touch.motion is not guaranteed."#]
            #[doc = r#"A wl_touch.down event is guaranteed to occur before the first"#]
            #[doc = r#"wl_touch.shape event for this touch ID but both events may occur within"#]
            #[doc = r#"the same wl_touch.frame."#]
            #[doc = r#""#]
            #[doc = r#"A touchpoint shape is approximated by an ellipse through the major and"#]
            #[doc = r#"minor axis length. The major axis length describes the longer diameter"#]
            #[doc = r#"of the ellipse, while the minor axis length describes the shorter"#]
            #[doc = r#"diameter. Major and minor are orthogonal and both are specified in"#]
            #[doc = r#"surface-local coordinates. The center of the ellipse is always at the"#]
            #[doc = r#"touchpoint location as reported by wl_touch.down or wl_touch.move."#]
            #[doc = r#""#]
            #[doc = r#"This event is only sent by the compositor if the touch device supports"#]
            #[doc = r#"shape reports. The client has to make reasonable assumptions about the"#]
            #[doc = r#"shape if it did not receive this event."#]
            async fn r#shape(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#id: i32,
                r#major: crate::wire::Fixed,
                r#minor: crate::wire::Fixed,
            ) -> crate::Result<()> {
                tracing::debug!("wl_touch -> shape");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_int(id)
                    .put_fixed(major)
                    .put_fixed(minor)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 5, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent when a touchpoint has changed its orientation."#]
            #[doc = r#""#]
            #[doc = r#"This event does not occur on its own. It is sent before a"#]
            #[doc = r#"wl_touch.frame event and carries the new shape information for"#]
            #[doc = r#"any previously reported, or new touch points of that frame."#]
            #[doc = r#""#]
            #[doc = r#"Other events describing the touch point such as wl_touch.down,"#]
            #[doc = r#"wl_touch.motion or wl_touch.shape may be sent within the"#]
            #[doc = r#"same wl_touch.frame. A client should treat these events as a single"#]
            #[doc = r#"logical touch point update. The order of wl_touch.shape,"#]
            #[doc = r#"wl_touch.orientation and wl_touch.motion is not guaranteed."#]
            #[doc = r#"A wl_touch.down event is guaranteed to occur before the first"#]
            #[doc = r#"wl_touch.orientation event for this touch ID but both events may occur"#]
            #[doc = r#"within the same wl_touch.frame."#]
            #[doc = r#""#]
            #[doc = r#"The orientation describes the clockwise angle of a touchpoint's major"#]
            #[doc = r#"axis to the positive surface y-axis and is normalized to the -180 to"#]
            #[doc = r#"+180 degree range. The granularity of orientation depends on the touch"#]
            #[doc = r#"device, some devices only support binary rotation values between 0 and"#]
            #[doc = r#"90 degrees."#]
            #[doc = r#""#]
            #[doc = r#"This event is only sent by the compositor if the touch device supports"#]
            #[doc = r#"orientation reports."#]
            async fn r#orientation(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#id: i32,
                r#orientation: crate::wire::Fixed,
            ) -> crate::Result<()> {
                tracing::debug!("wl_touch -> orientation");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_int(id)
                    .put_fixed(orientation)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 6, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod wl_output {
        enum r#Subpixel {}
        enum r#Transform {}
        enum r#Mode {}
        #[doc = r#"An output describes part of the compositor geometry.  The"#]
        #[doc = r#"compositor works in the 'compositor coordinate system' and an"#]
        #[doc = r#"output corresponds to a rectangular area in that space that is"#]
        #[doc = r#"actually visible.  This typically corresponds to a monitor that"#]
        #[doc = r#"displays part of the compositor space.  This object is published"#]
        #[doc = r#"as global during start up, or when a monitor is hotplugged."#]
        pub trait r#WlOutput {
            const INTERFACE: &'static str = "wl_output";
            const VERSION: u32 = 4;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_output -> release");
                        Self::r#release(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Using this request a client can tell the server that it is not going to"#]
            #[doc = r#"use the output object anymore."#]
            async fn r#release(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"The geometry event describes geometric properties of the output."#]
            #[doc = r#"The event is sent when binding to the output object and whenever"#]
            #[doc = r#"any of the properties change."#]
            #[doc = r#""#]
            #[doc = r#"The physical size can be set to zero if it doesn't make sense for this"#]
            #[doc = r#"output (e.g. for projectors or virtual outputs)."#]
            #[doc = r#""#]
            #[doc = r#"The geometry event will be followed by a done event (starting from"#]
            #[doc = r#"version 2)."#]
            #[doc = r#""#]
            #[doc = r#"Clients should use wl_surface.preferred_buffer_transform instead of the"#]
            #[doc = r#"transform advertised by this event to find the preferred buffer"#]
            #[doc = r#"transform to use for a surface."#]
            #[doc = r#""#]
            #[doc = r#"Note: wl_output only advertises partial information about the output"#]
            #[doc = r#"position and identification. Some compositors, for instance those not"#]
            #[doc = r#"implementing a desktop-style output layout or those exposing virtual"#]
            #[doc = r#"outputs, might fake this information. Instead of using x and y, clients"#]
            #[doc = r#"should use xdg_output.logical_position. Instead of using make and model,"#]
            #[doc = r#"clients should use name and description."#]
            async fn r#geometry(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#x: i32,
                r#y: i32,
                r#physical_width: i32,
                r#physical_height: i32,
                r#subpixel: i32,
                r#make: String,
                r#model: String,
                r#transform: i32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_output -> geometry");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .put_int(physical_width)
                    .put_int(physical_height)
                    .put_int(subpixel)
                    .put_string(Some(make))
                    .put_string(Some(model))
                    .put_int(transform)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"The mode event describes an available mode for the output."#]
            #[doc = r#""#]
            #[doc = r#"The event is sent when binding to the output object and there"#]
            #[doc = r#"will always be one mode, the current mode.  The event is sent"#]
            #[doc = r#"again if an output changes mode, for the mode that is now"#]
            #[doc = r#"current.  In other words, the current mode is always the last"#]
            #[doc = r#"mode that was received with the current flag set."#]
            #[doc = r#""#]
            #[doc = r#"Non-current modes are deprecated. A compositor can decide to only"#]
            #[doc = r#"advertise the current mode and never send other modes. Clients"#]
            #[doc = r#"should not rely on non-current modes."#]
            #[doc = r#""#]
            #[doc = r#"The size of a mode is given in physical hardware units of"#]
            #[doc = r#"the output device. This is not necessarily the same as"#]
            #[doc = r#"the output size in the global compositor space. For instance,"#]
            #[doc = r#"the output may be scaled, as described in wl_output.scale,"#]
            #[doc = r#"or transformed, as described in wl_output.transform. Clients"#]
            #[doc = r#"willing to retrieve the output size in the global compositor"#]
            #[doc = r#"space should use xdg_output.logical_size instead."#]
            #[doc = r#""#]
            #[doc = r#"The vertical refresh rate can be set to zero if it doesn't make"#]
            #[doc = r#"sense for this output (e.g. for virtual outputs)."#]
            #[doc = r#""#]
            #[doc = r#"The mode event will be followed by a done event (starting from"#]
            #[doc = r#"version 2)."#]
            #[doc = r#""#]
            #[doc = r#"Clients should not use the refresh rate to schedule frames. Instead,"#]
            #[doc = r#"they should use the wl_surface.frame event or the presentation-time"#]
            #[doc = r#"protocol."#]
            #[doc = r#""#]
            #[doc = r#"Note: this information is not always meaningful for all outputs. Some"#]
            #[doc = r#"compositors, such as those exposing virtual outputs, might fake the"#]
            #[doc = r#"refresh rate or the size."#]
            async fn r#mode(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#flags: u32,
                r#width: i32,
                r#height: i32,
                r#refresh: i32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_output -> mode");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(flags)
                    .put_int(width)
                    .put_int(height)
                    .put_int(refresh)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event is sent after all other properties have been"#]
            #[doc = r#"sent after binding to the output object and after any"#]
            #[doc = r#"other property changes done after that. This allows"#]
            #[doc = r#"changes to the output properties to be seen as"#]
            #[doc = r#"atomic, even if they happen via multiple events."#]
            async fn r#done(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("wl_output -> done");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event contains scaling geometry information"#]
            #[doc = r#"that is not in the geometry event. It may be sent after"#]
            #[doc = r#"binding the output object or if the output scale changes"#]
            #[doc = r#"later. The compositor will emit a non-zero, positive"#]
            #[doc = r#"value for scale. If it is not sent, the client should"#]
            #[doc = r#"assume a scale of 1."#]
            #[doc = r#""#]
            #[doc = r#"A scale larger than 1 means that the compositor will"#]
            #[doc = r#"automatically scale surface buffers by this amount"#]
            #[doc = r#"when rendering. This is used for very high resolution"#]
            #[doc = r#"displays where applications rendering at the native"#]
            #[doc = r#"resolution would be too small to be legible."#]
            #[doc = r#""#]
            #[doc = r#"Clients should use wl_surface.preferred_buffer_scale"#]
            #[doc = r#"instead of this event to find the preferred buffer"#]
            #[doc = r#"scale to use for a surface."#]
            #[doc = r#""#]
            #[doc = r#"The scale event will be followed by a done event."#]
            async fn r#scale(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#factor: i32,
            ) -> crate::Result<()> {
                tracing::debug!("wl_output -> scale");
                let payload = crate::wire::PayloadBuilder::new().put_int(factor).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 3, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Many compositors will assign user-friendly names to their outputs, show"#]
            #[doc = r#"them to the user, allow the user to refer to an output, etc. The client"#]
            #[doc = r#"may wish to know this name as well to offer the user similar behaviors."#]
            #[doc = r#""#]
            #[doc = r#"The name is a UTF-8 string with no convention defined for its contents."#]
            #[doc = r#"Each name is unique among all wl_output globals. The name is only"#]
            #[doc = r#"guaranteed to be unique for the compositor instance."#]
            #[doc = r#""#]
            #[doc = r#"The same output name is used for all clients for a given wl_output"#]
            #[doc = r#"global. Thus, the name can be shared across processes to refer to a"#]
            #[doc = r#"specific wl_output global."#]
            #[doc = r#""#]
            #[doc = r#"The name is not guaranteed to be persistent across sessions, thus cannot"#]
            #[doc = r#"be used to reliably identify an output in e.g. configuration files."#]
            #[doc = r#""#]
            #[doc = r#"Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do"#]
            #[doc = r#"not assume that the name is a reflection of an underlying DRM connector,"#]
            #[doc = r#"X11 connection, etc."#]
            #[doc = r#""#]
            #[doc = r#"The name event is sent after binding the output object. This event is"#]
            #[doc = r#"only sent once per output object, and the name does not change over the"#]
            #[doc = r#"lifetime of the wl_output global."#]
            #[doc = r#""#]
            #[doc = r#"Compositors may re-use the same output name if the wl_output global is"#]
            #[doc = r#"destroyed and re-created later. Compositors should avoid re-using the"#]
            #[doc = r#"same name if possible."#]
            #[doc = r#""#]
            #[doc = r#"The name event will be followed by a done event."#]
            async fn r#name(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#name: String,
            ) -> crate::Result<()> {
                tracing::debug!("wl_output -> name");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_string(Some(name))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 4, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Many compositors can produce human-readable descriptions of their"#]
            #[doc = r#"outputs. The client may wish to know this description as well, e.g. for"#]
            #[doc = r#"output selection purposes."#]
            #[doc = r#""#]
            #[doc = r#"The description is a UTF-8 string with no convention defined for its"#]
            #[doc = r#"contents. The description is not guaranteed to be unique among all"#]
            #[doc = r#"wl_output globals. Examples might include 'Foocorp 11" Display' or"#]
            #[doc = r#"'Virtual X11 output via :1'."#]
            #[doc = r#""#]
            #[doc = r#"The description event is sent after binding the output object and"#]
            #[doc = r#"whenever the description changes. The description is optional, and may"#]
            #[doc = r#"not be sent at all."#]
            #[doc = r#""#]
            #[doc = r#"The description event will be followed by a done event."#]
            async fn r#description(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#description: String,
            ) -> crate::Result<()> {
                tracing::debug!("wl_output -> description");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_string(Some(description))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 5, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod wl_region {
        #[doc = r#"A region object describes an area."#]
        #[doc = r#""#]
        #[doc = r#"Region objects are used to describe the opaque and input"#]
        #[doc = r#"regions of a surface."#]
        pub trait r#WlRegion {
            const INTERFACE: &'static str = "wl_region";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_region -> destroy");
                        Self::r#destroy(client).await
                    }
                    1 => {
                        tracing::debug!("wl_region -> add");
                        Self::r#add(
                            client,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    2 => {
                        tracing::debug!("wl_region -> subtract");
                        Self::r#subtract(
                            client,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Destroy the region.  This will invalidate the object ID."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Add the specified rectangle to the region."#]
            async fn r#add(
                client: &mut crate::Client,
                r#x: i32,
                r#y: i32,
                r#width: i32,
                r#height: i32,
            ) -> crate::Result<()>;
            #[doc = r#"Subtract the specified rectangle from the region."#]
            async fn r#subtract(
                client: &mut crate::Client,
                r#x: i32,
                r#y: i32,
                r#width: i32,
                r#height: i32,
            ) -> crate::Result<()>;
        }
    }
    pub mod wl_subcompositor {
        enum r#Error {}
        #[doc = r#"The global interface exposing sub-surface compositing capabilities."#]
        #[doc = r#"A wl_surface, that has sub-surfaces associated, is called the"#]
        #[doc = r#"parent surface. Sub-surfaces can be arbitrarily nested and create"#]
        #[doc = r#"a tree of sub-surfaces."#]
        #[doc = r#""#]
        #[doc = r#"The root surface in a tree of sub-surfaces is the main"#]
        #[doc = r#"surface. The main surface cannot be a sub-surface, because"#]
        #[doc = r#"sub-surfaces must always have a parent."#]
        #[doc = r#""#]
        #[doc = r#"A main surface with its sub-surfaces forms a (compound) window."#]
        #[doc = r#"For window management purposes, this set of wl_surface objects is"#]
        #[doc = r#"to be considered as a single window, and it should also behave as"#]
        #[doc = r#"such."#]
        #[doc = r#""#]
        #[doc = r#"The aim of sub-surfaces is to offload some of the compositing work"#]
        #[doc = r#"within a window from clients to the compositor. A prime example is"#]
        #[doc = r#"a video player with decorations and video in separate wl_surface"#]
        #[doc = r#"objects. This should allow the compositor to pass YUV video buffer"#]
        #[doc = r#"processing to dedicated overlay hardware when possible."#]
        pub trait r#WlSubcompositor {
            const INTERFACE: &'static str = "wl_subcompositor";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_subcompositor -> destroy");
                        Self::r#destroy(client).await
                    }
                    1 => {
                        tracing::debug!("wl_subcompositor -> get_subsurface");
                        Self::r#get_subsurface(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Informs the server that the client will not be using this"#]
            #[doc = r#"protocol object anymore. This does not affect any other"#]
            #[doc = r#"objects, wl_subsurface objects included."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Create a sub-surface interface for the given surface, and"#]
            #[doc = r#"associate it with the given parent surface. This turns a"#]
            #[doc = r#"plain wl_surface into a sub-surface."#]
            #[doc = r#""#]
            #[doc = r#"The to-be sub-surface must not already have another role, and it"#]
            #[doc = r#"must not have an existing wl_subsurface object. Otherwise the"#]
            #[doc = r#"bad_surface protocol error is raised."#]
            #[doc = r#""#]
            #[doc = r#"Adding sub-surfaces to a parent is a double-buffered operation on the"#]
            #[doc = r#"parent (see wl_surface.commit). The effect of adding a sub-surface"#]
            #[doc = r#"becomes visible on the next time the state of the parent surface is"#]
            #[doc = r#"applied."#]
            #[doc = r#""#]
            #[doc = r#"The parent surface must not be one of the child surface's descendants,"#]
            #[doc = r#"and the parent must be different from the child surface, otherwise the"#]
            #[doc = r#"bad_parent protocol error is raised."#]
            #[doc = r#""#]
            #[doc = r#"This request modifies the behaviour of wl_surface.commit request on"#]
            #[doc = r#"the sub-surface, see the documentation on wl_subsurface interface."#]
            async fn r#get_subsurface(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
                r#surface: crate::wire::ObjectId,
                r#parent: crate::wire::ObjectId,
            ) -> crate::Result<()>;
        }
    }
    pub mod wl_subsurface {
        enum r#Error {}
        #[doc = r#"An additional interface to a wl_surface object, which has been"#]
        #[doc = r#"made a sub-surface. A sub-surface has one parent surface. A"#]
        #[doc = r#"sub-surface's size and position are not limited to that of the parent."#]
        #[doc = r#"Particularly, a sub-surface is not automatically clipped to its"#]
        #[doc = r#"parent's area."#]
        #[doc = r#""#]
        #[doc = r#"A sub-surface becomes mapped, when a non-NULL wl_buffer is applied"#]
        #[doc = r#"and the parent surface is mapped. The order of which one happens"#]
        #[doc = r#"first is irrelevant. A sub-surface is hidden if the parent becomes"#]
        #[doc = r#"hidden, or if a NULL wl_buffer is applied. These rules apply"#]
        #[doc = r#"recursively through the tree of surfaces."#]
        #[doc = r#""#]
        #[doc = r#"The behaviour of a wl_surface.commit request on a sub-surface"#]
        #[doc = r#"depends on the sub-surface's mode. The possible modes are"#]
        #[doc = r#"synchronized and desynchronized, see methods"#]
        #[doc = r#"wl_subsurface.set_sync and wl_subsurface.set_desync. Synchronized"#]
        #[doc = r#"mode caches the wl_surface state to be applied when the parent's"#]
        #[doc = r#"state gets applied, and desynchronized mode applies the pending"#]
        #[doc = r#"wl_surface state directly. A sub-surface is initially in the"#]
        #[doc = r#"synchronized mode."#]
        #[doc = r#""#]
        #[doc = r#"Sub-surfaces also have another kind of state, which is managed by"#]
        #[doc = r#"wl_subsurface requests, as opposed to wl_surface requests. This"#]
        #[doc = r#"state includes the sub-surface position relative to the parent"#]
        #[doc = r#"surface (wl_subsurface.set_position), and the stacking order of"#]
        #[doc = r#"the parent and its sub-surfaces (wl_subsurface.place_above and"#]
        #[doc = r#".place_below). This state is applied when the parent surface's"#]
        #[doc = r#"wl_surface state is applied, regardless of the sub-surface's mode."#]
        #[doc = r#"As the exception, set_sync and set_desync are effective immediately."#]
        #[doc = r#""#]
        #[doc = r#"The main surface can be thought to be always in desynchronized mode,"#]
        #[doc = r#"since it does not have a parent in the sub-surfaces sense."#]
        #[doc = r#""#]
        #[doc = r#"Even if a sub-surface is in desynchronized mode, it will behave as"#]
        #[doc = r#"in synchronized mode, if its parent surface behaves as in"#]
        #[doc = r#"synchronized mode. This rule is applied recursively throughout the"#]
        #[doc = r#"tree of surfaces. This means, that one can set a sub-surface into"#]
        #[doc = r#"synchronized mode, and then assume that all its child and grand-child"#]
        #[doc = r#"sub-surfaces are synchronized, too, without explicitly setting them."#]
        #[doc = r#""#]
        #[doc = r#"Destroying a sub-surface takes effect immediately. If you need to"#]
        #[doc = r#"synchronize the removal of a sub-surface to the parent surface update,"#]
        #[doc = r#"unmap the sub-surface first by attaching a NULL wl_buffer, update parent,"#]
        #[doc = r#"and then destroy the sub-surface."#]
        #[doc = r#""#]
        #[doc = r#"If the parent wl_surface object is destroyed, the sub-surface is"#]
        #[doc = r#"unmapped."#]
        #[doc = r#""#]
        #[doc = r#"A sub-surface never has the keyboard focus of any seat."#]
        #[doc = r#""#]
        #[doc = r#"The wl_surface.offset request is ignored: clients must use set_position"#]
        #[doc = r#"instead to move the sub-surface."#]
        pub trait r#WlSubsurface {
            const INTERFACE: &'static str = "wl_subsurface";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wl_subsurface -> destroy");
                        Self::r#destroy(client).await
                    }
                    1 => {
                        tracing::debug!("wl_subsurface -> set_position");
                        Self::r#set_position(client, message.int()?, message.int()?).await
                    }
                    2 => {
                        tracing::debug!("wl_subsurface -> place_above");
                        Self::r#place_above(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    3 => {
                        tracing::debug!("wl_subsurface -> place_below");
                        Self::r#place_below(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    4 => {
                        tracing::debug!("wl_subsurface -> set_sync");
                        Self::r#set_sync(client).await
                    }
                    5 => {
                        tracing::debug!("wl_subsurface -> set_desync");
                        Self::r#set_desync(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"The sub-surface interface is removed from the wl_surface object"#]
            #[doc = r#"that was turned into a sub-surface with a"#]
            #[doc = r#"wl_subcompositor.get_subsurface request. The wl_surface's association"#]
            #[doc = r#"to the parent is deleted. The wl_surface is unmapped immediately."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"This schedules a sub-surface position change."#]
            #[doc = r#"The sub-surface will be moved so that its origin (top left"#]
            #[doc = r#"corner pixel) will be at the location x, y of the parent surface"#]
            #[doc = r#"coordinate system. The coordinates are not restricted to the parent"#]
            #[doc = r#"surface area. Negative values are allowed."#]
            #[doc = r#""#]
            #[doc = r#"The scheduled coordinates will take effect whenever the state of the"#]
            #[doc = r#"parent surface is applied."#]
            #[doc = r#""#]
            #[doc = r#"If more than one set_position request is invoked by the client before"#]
            #[doc = r#"the commit of the parent surface, the position of a new request always"#]
            #[doc = r#"replaces the scheduled position from any previous request."#]
            #[doc = r#""#]
            #[doc = r#"The initial position is 0, 0."#]
            async fn r#set_position(
                client: &mut crate::Client,
                r#x: i32,
                r#y: i32,
            ) -> crate::Result<()>;
            #[doc = r#"This sub-surface is taken from the stack, and put back just"#]
            #[doc = r#"above the reference surface, changing the z-order of the sub-surfaces."#]
            #[doc = r#"The reference surface must be one of the sibling surfaces, or the"#]
            #[doc = r#"parent surface. Using any other surface, including this sub-surface,"#]
            #[doc = r#"will cause a protocol error."#]
            #[doc = r#""#]
            #[doc = r#"The z-order is double-buffered. Requests are handled in order and"#]
            #[doc = r#"applied immediately to a pending state. The final pending state is"#]
            #[doc = r#"copied to the active state the next time the state of the parent"#]
            #[doc = r#"surface is applied."#]
            #[doc = r#""#]
            #[doc = r#"A new sub-surface is initially added as the top-most in the stack"#]
            #[doc = r#"of its siblings and parent."#]
            async fn r#place_above(
                client: &mut crate::Client,
                r#sibling: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"The sub-surface is placed just below the reference surface."#]
            #[doc = r#"See wl_subsurface.place_above."#]
            async fn r#place_below(
                client: &mut crate::Client,
                r#sibling: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"Change the commit behaviour of the sub-surface to synchronized"#]
            #[doc = r#"mode, also described as the parent dependent mode."#]
            #[doc = r#""#]
            #[doc = r#"In synchronized mode, wl_surface.commit on a sub-surface will"#]
            #[doc = r#"accumulate the committed state in a cache, but the state will"#]
            #[doc = r#"not be applied and hence will not change the compositor output."#]
            #[doc = r#"The cached state is applied to the sub-surface immediately after"#]
            #[doc = r#"the parent surface's state is applied. This ensures atomic"#]
            #[doc = r#"updates of the parent and all its synchronized sub-surfaces."#]
            #[doc = r#"Applying the cached state will invalidate the cache, so further"#]
            #[doc = r#"parent surface commits do not (re-)apply old state."#]
            #[doc = r#""#]
            #[doc = r#"See wl_subsurface for the recursive effect of this mode."#]
            async fn r#set_sync(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Change the commit behaviour of the sub-surface to desynchronized"#]
            #[doc = r#"mode, also described as independent or freely running mode."#]
            #[doc = r#""#]
            #[doc = r#"In desynchronized mode, wl_surface.commit on a sub-surface will"#]
            #[doc = r#"apply the pending state directly, without caching, as happens"#]
            #[doc = r#"normally with a wl_surface. Calling wl_surface.commit on the"#]
            #[doc = r#"parent surface has no effect on the sub-surface's wl_surface"#]
            #[doc = r#"state. This mode allows a sub-surface to be updated on its own."#]
            #[doc = r#""#]
            #[doc = r#"If cached state exists when wl_surface.commit is called in"#]
            #[doc = r#"desynchronized mode, the pending state is added to the cached"#]
            #[doc = r#"state, and applied as a whole. This invalidates the cache."#]
            #[doc = r#""#]
            #[doc = r#"Note: even if a sub-surface is set to desynchronized, a parent"#]
            #[doc = r#"sub-surface may override it to behave as synchronized. For details,"#]
            #[doc = r#"see wl_subsurface."#]
            #[doc = r#""#]
            #[doc = r#"If a surface's parent surface behaves as desynchronized, then"#]
            #[doc = r#"the cached state is applied on set_desync."#]
            async fn r#set_desync(client: &mut crate::Client) -> crate::Result<()>;
        }
    }
}
pub mod linux_dmabuf_v1 {
    pub mod zwp_linux_dmabuf_v1 {
        #[doc = r#"Following the interfaces from:"#]
        #[doc = r#"https://www.khronos.org/registry/egl/extensions/EXT/EGL_EXT_image_dma_buf_import.txt"#]
        #[doc = r#"https://www.khronos.org/registry/EGL/extensions/EXT/EGL_EXT_image_dma_buf_import_modifiers.txt"#]
        #[doc = r#"and the Linux DRM sub-system's AddFb2 ioctl."#]
        #[doc = r#""#]
        #[doc = r#"This interface offers ways to create generic dmabuf-based wl_buffers."#]
        #[doc = r#""#]
        #[doc = r#"Clients can use the get_surface_feedback request to get dmabuf feedback"#]
        #[doc = r#"for a particular surface. If the client wants to retrieve feedback not"#]
        #[doc = r#"tied to a surface, they can use the get_default_feedback request."#]
        #[doc = r#""#]
        #[doc = r#"The following are required from clients:"#]
        #[doc = r#""#]
        #[doc = r#"- Clients must ensure that either all data in the dma-buf is"#]
        #[doc = r#"coherent for all subsequent read access or that coherency is"#]
        #[doc = r#"correctly handled by the underlying kernel-side dma-buf"#]
        #[doc = r#"implementation."#]
        #[doc = r#""#]
        #[doc = r#"- Don't make any more attachments after sending the buffer to the"#]
        #[doc = r#"compositor. Making more attachments later increases the risk of"#]
        #[doc = r#"the compositor not being able to use (re-import) an existing"#]
        #[doc = r#"dmabuf-based wl_buffer."#]
        #[doc = r#""#]
        #[doc = r#"The underlying graphics stack must ensure the following:"#]
        #[doc = r#""#]
        #[doc = r#"- The dmabuf file descriptors relayed to the server will stay valid"#]
        #[doc = r#"for the whole lifetime of the wl_buffer. This means the server may"#]
        #[doc = r#"at any time use those fds to import the dmabuf into any kernel"#]
        #[doc = r#"sub-system that might accept it."#]
        #[doc = r#""#]
        #[doc = r#"However, when the underlying graphics stack fails to deliver the"#]
        #[doc = r#"promise, because of e.g. a device hot-unplug which raises internal"#]
        #[doc = r#"errors, after the wl_buffer has been successfully created the"#]
        #[doc = r#"compositor must not raise protocol errors to the client when dmabuf"#]
        #[doc = r#"import later fails."#]
        #[doc = r#""#]
        #[doc = r#"To create a wl_buffer from one or more dmabufs, a client creates a"#]
        #[doc = r#"zwp_linux_dmabuf_params_v1 object with a zwp_linux_dmabuf_v1.create_params"#]
        #[doc = r#"request. All planes required by the intended format are added with"#]
        #[doc = r#"the 'add' request. Finally, a 'create' or 'create_immed' request is"#]
        #[doc = r#"issued, which has the following outcome depending on the import success."#]
        #[doc = r#""#]
        #[doc = r#"The 'create' request,"#]
        #[doc = r#"- on success, triggers a 'created' event which provides the final"#]
        #[doc = r#"wl_buffer to the client."#]
        #[doc = r#"- on failure, triggers a 'failed' event to convey that the server"#]
        #[doc = r#"cannot use the dmabufs received from the client."#]
        #[doc = r#""#]
        #[doc = r#"For the 'create_immed' request,"#]
        #[doc = r#"- on success, the server immediately imports the added dmabufs to"#]
        #[doc = r#"create a wl_buffer. No event is sent from the server in this case."#]
        #[doc = r#"- on failure, the server can choose to either:"#]
        #[doc = r#"- terminate the client by raising a fatal error."#]
        #[doc = r#"- mark the wl_buffer as failed, and send a 'failed' event to the"#]
        #[doc = r#"client. If the client uses a failed wl_buffer as an argument to any"#]
        #[doc = r#"request, the behaviour is compositor implementation-defined."#]
        #[doc = r#""#]
        #[doc = r#"For all DRM formats and unless specified in another protocol extension,"#]
        #[doc = r#"pre-multiplied alpha is used for pixel values."#]
        #[doc = r#""#]
        #[doc = r#"Unless specified otherwise in another protocol extension, implicit"#]
        #[doc = r#"synchronization is used. In other words, compositors and clients must"#]
        #[doc = r#"wait and signal fences implicitly passed via the DMA-BUF's reservation"#]
        #[doc = r#"mechanism."#]
        pub trait r#ZwpLinuxDmabufV1 {
            const INTERFACE: &'static str = "zwp_linux_dmabuf_v1";
            const VERSION: u32 = 5;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("zwp_linux_dmabuf_v1 -> destroy");
                        Self::r#destroy(client).await
                    }
                    1 => {
                        tracing::debug!("zwp_linux_dmabuf_v1 -> create_params");
                        Self::r#create_params(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2 => {
                        tracing::debug!("zwp_linux_dmabuf_v1 -> get_default_feedback");
                        Self::r#get_default_feedback(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    3 => {
                        tracing::debug!("zwp_linux_dmabuf_v1 -> get_surface_feedback");
                        Self::r#get_surface_feedback(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Objects created through this interface, especially wl_buffers, will"#]
            #[doc = r#"remain valid."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"This temporary object is used to collect multiple dmabuf handles into"#]
            #[doc = r#"a single batch to create a wl_buffer. It can only be used once and"#]
            #[doc = r#"should be destroyed after a 'created' or 'failed' event has been"#]
            #[doc = r#"received."#]
            async fn r#create_params(
                client: &mut crate::Client,
                r#params_id: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"This request creates a new wp_linux_dmabuf_feedback object not bound"#]
            #[doc = r#"to a particular surface. This object will deliver feedback about dmabuf"#]
            #[doc = r#"parameters to use if the client doesn't support per-surface feedback"#]
            #[doc = r#"(see get_surface_feedback)."#]
            async fn r#get_default_feedback(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"This request creates a new wp_linux_dmabuf_feedback object for the"#]
            #[doc = r#"specified wl_surface. This object will deliver feedback about dmabuf"#]
            #[doc = r#"parameters to use for buffers attached to this surface."#]
            #[doc = r#""#]
            #[doc = r#"If the surface is destroyed before the wp_linux_dmabuf_feedback object,"#]
            #[doc = r#"the feedback object becomes inert."#]
            async fn r#get_surface_feedback(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
                r#surface: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"This event advertises one buffer format that the server supports."#]
            #[doc = r#"All the supported formats are advertised once when the client"#]
            #[doc = r#"binds to this interface. A roundtrip after binding guarantees"#]
            #[doc = r#"that the client has received all supported formats."#]
            #[doc = r#""#]
            #[doc = r#"For the definition of the format codes, see the"#]
            #[doc = r#"zwp_linux_buffer_params_v1::create request."#]
            #[doc = r#""#]
            #[doc = r#"Starting version 4, the format event is deprecated and must not be"#]
            #[doc = r#"sent by compositors. Instead, use get_default_feedback or"#]
            #[doc = r#"get_surface_feedback."#]
            async fn r#format(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#format: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_linux_dmabuf_v1 -> format");
                let payload = crate::wire::PayloadBuilder::new().put_uint(format).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event advertises the formats that the server supports, along with"#]
            #[doc = r#"the modifiers supported for each format. All the supported modifiers"#]
            #[doc = r#"for all the supported formats are advertised once when the client"#]
            #[doc = r#"binds to this interface. A roundtrip after binding guarantees that"#]
            #[doc = r#"the client has received all supported format-modifier pairs."#]
            #[doc = r#""#]
            #[doc = r#"For legacy support, DRM_FORMAT_MOD_INVALID (that is, modifier_hi =="#]
            #[doc = r#"0x00ffffff and modifier_lo == 0xffffffff) is allowed in this event."#]
            #[doc = r#"It indicates that the server can support the format with an implicit"#]
            #[doc = r#"modifier. When a plane has DRM_FORMAT_MOD_INVALID as its modifier, it"#]
            #[doc = r#"is as if no explicit modifier is specified. The effective modifier"#]
            #[doc = r#"will be derived from the dmabuf."#]
            #[doc = r#""#]
            #[doc = r#"A compositor that sends valid modifiers and DRM_FORMAT_MOD_INVALID for"#]
            #[doc = r#"a given format supports both explicit modifiers and implicit modifiers."#]
            #[doc = r#""#]
            #[doc = r#"For the definition of the format and modifier codes, see the"#]
            #[doc = r#"zwp_linux_buffer_params_v1::create and zwp_linux_buffer_params_v1::add"#]
            #[doc = r#"requests."#]
            #[doc = r#""#]
            #[doc = r#"Starting version 4, the modifier event is deprecated and must not be"#]
            #[doc = r#"sent by compositors. Instead, use get_default_feedback or"#]
            #[doc = r#"get_surface_feedback."#]
            async fn r#modifier(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#format: u32,
                r#modifier_hi: u32,
                r#modifier_lo: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_linux_dmabuf_v1 -> modifier");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(format)
                    .put_uint(modifier_hi)
                    .put_uint(modifier_lo)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod zwp_linux_buffer_params_v1 {
        enum r#Error {}
        enum r#Flags {}
        #[doc = r#"This temporary object is a collection of dmabufs and other"#]
        #[doc = r#"parameters that together form a single logical buffer. The temporary"#]
        #[doc = r#"object may eventually create one wl_buffer unless cancelled by"#]
        #[doc = r#"destroying it before requesting 'create'."#]
        #[doc = r#""#]
        #[doc = r#"Single-planar formats only require one dmabuf, however"#]
        #[doc = r#"multi-planar formats may require more than one dmabuf. For all"#]
        #[doc = r#"formats, an 'add' request must be called once per plane (even if the"#]
        #[doc = r#"underlying dmabuf fd is identical)."#]
        #[doc = r#""#]
        #[doc = r#"You must use consecutive plane indices ('plane_idx' argument for 'add')"#]
        #[doc = r#"from zero to the number of planes used by the drm_fourcc format code."#]
        #[doc = r#"All planes required by the format must be given exactly once, but can"#]
        #[doc = r#"be given in any order. Each plane index can be set only once."#]
        pub trait r#ZwpLinuxBufferParamsV1 {
            const INTERFACE: &'static str = "zwp_linux_buffer_params_v1";
            const VERSION: u32 = 5;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("zwp_linux_buffer_params_v1 -> destroy");
                        Self::r#destroy(client).await
                    }
                    1 => {
                        tracing::debug!("zwp_linux_buffer_params_v1 -> add");
                        Self::r#add(
                            client,
                            message.int()?,
                            message.uint()?,
                            message.uint()?,
                            message.uint()?,
                            message.uint()?,
                            message.uint()?,
                        )
                        .await
                    }
                    2 => {
                        tracing::debug!("zwp_linux_buffer_params_v1 -> create");
                        Self::r#create(
                            client,
                            message.int()?,
                            message.int()?,
                            message.uint()?,
                            message.uint()?,
                        )
                        .await
                    }
                    3 => {
                        tracing::debug!("zwp_linux_buffer_params_v1 -> create_immed");
                        Self::r#create_immed(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.int()?,
                            message.int()?,
                            message.uint()?,
                            message.uint()?,
                        )
                        .await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Cleans up the temporary data sent to the server for dmabuf-based"#]
            #[doc = r#"wl_buffer creation."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"This request adds one dmabuf to the set in this"#]
            #[doc = r#"zwp_linux_buffer_params_v1."#]
            #[doc = r#""#]
            #[doc = r#"The 64-bit unsigned value combined from modifier_hi and modifier_lo"#]
            #[doc = r#"is the dmabuf layout modifier. DRM AddFB2 ioctl calls this the"#]
            #[doc = r#"fb modifier, which is defined in drm_mode.h of Linux UAPI."#]
            #[doc = r#"This is an opaque token. Drivers use this token to express tiling,"#]
            #[doc = r#"compression, etc. driver-specific modifications to the base format"#]
            #[doc = r#"defined by the DRM fourcc code."#]
            #[doc = r#""#]
            #[doc = r#"Starting from version 4, the invalid_format protocol error is sent if"#]
            #[doc = r#"the format + modifier pair was not advertised as supported."#]
            #[doc = r#""#]
            #[doc = r#"Starting from version 5, the invalid_format protocol error is sent if"#]
            #[doc = r#"all planes don't use the same modifier."#]
            #[doc = r#""#]
            #[doc = r#"This request raises the PLANE_IDX error if plane_idx is too large."#]
            #[doc = r#"The error PLANE_SET is raised if attempting to set a plane that"#]
            #[doc = r#"was already set."#]
            async fn r#add(
                client: &mut crate::Client,
                r#fd: std::os::fd::RawFd,
                r#plane_idx: u32,
                r#offset: u32,
                r#stride: u32,
                r#modifier_hi: u32,
                r#modifier_lo: u32,
            ) -> crate::Result<()>;
            #[doc = r#"This asks for creation of a wl_buffer from the added dmabuf"#]
            #[doc = r#"buffers. The wl_buffer is not created immediately but returned via"#]
            #[doc = r#"the 'created' event if the dmabuf sharing succeeds. The sharing"#]
            #[doc = r#"may fail at runtime for reasons a client cannot predict, in"#]
            #[doc = r#"which case the 'failed' event is triggered."#]
            #[doc = r#""#]
            #[doc = r#"The 'format' argument is a DRM_FORMAT code, as defined by the"#]
            #[doc = r#"libdrm's drm_fourcc.h. The Linux kernel's DRM sub-system is the"#]
            #[doc = r#"authoritative source on how the format codes should work."#]
            #[doc = r#""#]
            #[doc = r#"The 'flags' is a bitfield of the flags defined in enum "flags"."#]
            #[doc = r#"'y_invert' means the that the image needs to be y-flipped."#]
            #[doc = r#""#]
            #[doc = r#"Flag 'interlaced' means that the frame in the buffer is not"#]
            #[doc = r#"progressive as usual, but interlaced. An interlaced buffer as"#]
            #[doc = r#"supported here must always contain both top and bottom fields."#]
            #[doc = r#"The top field always begins on the first pixel row. The temporal"#]
            #[doc = r#"ordering between the two fields is top field first, unless"#]
            #[doc = r#"'bottom_first' is specified. It is undefined whether 'bottom_first'"#]
            #[doc = r#"is ignored if 'interlaced' is not set."#]
            #[doc = r#""#]
            #[doc = r#"This protocol does not convey any information about field rate,"#]
            #[doc = r#"duration, or timing, other than the relative ordering between the"#]
            #[doc = r#"two fields in one buffer. A compositor may have to estimate the"#]
            #[doc = r#"intended field rate from the incoming buffer rate. It is undefined"#]
            #[doc = r#"whether the time of receiving wl_surface.commit with a new buffer"#]
            #[doc = r#"attached, applying the wl_surface state, wl_surface.frame callback"#]
            #[doc = r#"trigger, presentation, or any other point in the compositor cycle"#]
            #[doc = r#"is used to measure the frame or field times. There is no support"#]
            #[doc = r#"for detecting missed or late frames/fields/buffers either, and"#]
            #[doc = r#"there is no support whatsoever for cooperating with interlaced"#]
            #[doc = r#"compositor output."#]
            #[doc = r#""#]
            #[doc = r#"The composited image quality resulting from the use of interlaced"#]
            #[doc = r#"buffers is explicitly undefined. A compositor may use elaborate"#]
            #[doc = r#"hardware features or software to deinterlace and create progressive"#]
            #[doc = r#"output frames from a sequence of interlaced input buffers, or it"#]
            #[doc = r#"may produce substandard image quality. However, compositors that"#]
            #[doc = r#"cannot guarantee reasonable image quality in all cases are recommended"#]
            #[doc = r#"to just reject all interlaced buffers."#]
            #[doc = r#""#]
            #[doc = r#"Any argument errors, including non-positive width or height,"#]
            #[doc = r#"mismatch between the number of planes and the format, bad"#]
            #[doc = r#"format, bad offset or stride, may be indicated by fatal protocol"#]
            #[doc = r#"errors: INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS,"#]
            #[doc = r#"OUT_OF_BOUNDS."#]
            #[doc = r#""#]
            #[doc = r#"Dmabuf import errors in the server that are not obvious client"#]
            #[doc = r#"bugs are returned via the 'failed' event as non-fatal. This"#]
            #[doc = r#"allows attempting dmabuf sharing and falling back in the client"#]
            #[doc = r#"if it fails."#]
            #[doc = r#""#]
            #[doc = r#"This request can be sent only once in the object's lifetime, after"#]
            #[doc = r#"which the only legal request is destroy. This object should be"#]
            #[doc = r#"destroyed after issuing a 'create' request. Attempting to use this"#]
            #[doc = r#"object after issuing 'create' raises ALREADY_USED protocol error."#]
            #[doc = r#""#]
            #[doc = r#"It is not mandatory to issue 'create'. If a client wants to"#]
            #[doc = r#"cancel the buffer creation, it can just destroy this object."#]
            async fn r#create(
                client: &mut crate::Client,
                r#width: i32,
                r#height: i32,
                r#format: u32,
                r#flags: u32,
            ) -> crate::Result<()>;
            #[doc = r#"This asks for immediate creation of a wl_buffer by importing the"#]
            #[doc = r#"added dmabufs."#]
            #[doc = r#""#]
            #[doc = r#"In case of import success, no event is sent from the server, and the"#]
            #[doc = r#"wl_buffer is ready to be used by the client."#]
            #[doc = r#""#]
            #[doc = r#"Upon import failure, either of the following may happen, as seen fit"#]
            #[doc = r#"by the implementation:"#]
            #[doc = r#"- the client is terminated with one of the following fatal protocol"#]
            #[doc = r#"errors:"#]
            #[doc = r#"- INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS, OUT_OF_BOUNDS,"#]
            #[doc = r#"in case of argument errors such as mismatch between the number"#]
            #[doc = r#"of planes and the format, bad format, non-positive width or"#]
            #[doc = r#"height, or bad offset or stride."#]
            #[doc = r#"- INVALID_WL_BUFFER, in case the cause for failure is unknown or"#]
            #[doc = r#"plaform specific."#]
            #[doc = r#"- the server creates an invalid wl_buffer, marks it as failed and"#]
            #[doc = r#"sends a 'failed' event to the client. The result of using this"#]
            #[doc = r#"invalid wl_buffer as an argument in any request by the client is"#]
            #[doc = r#"defined by the compositor implementation."#]
            #[doc = r#""#]
            #[doc = r#"This takes the same arguments as a 'create' request, and obeys the"#]
            #[doc = r#"same restrictions."#]
            async fn r#create_immed(
                client: &mut crate::Client,
                r#buffer_id: crate::wire::ObjectId,
                r#width: i32,
                r#height: i32,
                r#format: u32,
                r#flags: u32,
            ) -> crate::Result<()>;
            #[doc = r#"This event indicates that the attempted buffer creation was"#]
            #[doc = r#"successful. It provides the new wl_buffer referencing the dmabuf(s)."#]
            #[doc = r#""#]
            #[doc = r#"Upon receiving this event, the client should destroy the"#]
            #[doc = r#"zwp_linux_buffer_params_v1 object."#]
            async fn r#created(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#buffer: crate::wire::ObjectId,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_linux_buffer_params_v1 -> created");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_object(Some(buffer))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event indicates that the attempted buffer creation has"#]
            #[doc = r#"failed. It usually means that one of the dmabuf constraints"#]
            #[doc = r#"has not been fulfilled."#]
            #[doc = r#""#]
            #[doc = r#"Upon receiving this event, the client should destroy the"#]
            #[doc = r#"zwp_linux_buffer_params_v1 object."#]
            async fn r#failed(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_linux_buffer_params_v1 -> failed");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod zwp_linux_dmabuf_feedback_v1 {
        enum r#TrancheFlags {}
        #[doc = r#"This object advertises dmabuf parameters feedback. This includes the"#]
        #[doc = r#"preferred devices and the supported formats/modifiers."#]
        #[doc = r#""#]
        #[doc = r#"The parameters are sent once when this object is created and whenever they"#]
        #[doc = r#"change. The done event is always sent once after all parameters have been"#]
        #[doc = r#"sent. When a single parameter changes, all parameters are re-sent by the"#]
        #[doc = r#"compositor."#]
        #[doc = r#""#]
        #[doc = r#"Compositors can re-send the parameters when the current client buffer"#]
        #[doc = r#"allocations are sub-optimal. Compositors should not re-send the"#]
        #[doc = r#"parameters if re-allocating the buffers would not result in a more optimal"#]
        #[doc = r#"configuration. In particular, compositors should avoid sending the exact"#]
        #[doc = r#"same parameters multiple times in a row."#]
        #[doc = r#""#]
        #[doc = r#"The tranche_target_device and tranche_formats events are grouped by"#]
        #[doc = r#"tranches of preference. For each tranche, a tranche_target_device, one"#]
        #[doc = r#"tranche_flags and one or more tranche_formats events are sent, followed"#]
        #[doc = r#"by a tranche_done event finishing the list. The tranches are sent in"#]
        #[doc = r#"descending order of preference. All formats and modifiers in the same"#]
        #[doc = r#"tranche have the same preference."#]
        #[doc = r#""#]
        #[doc = r#"To send parameters, the compositor sends one main_device event, tranches"#]
        #[doc = r#"(each consisting of one tranche_target_device event, one tranche_flags"#]
        #[doc = r#"event, tranche_formats events and then a tranche_done event), then one"#]
        #[doc = r#"done event."#]
        pub trait r#ZwpLinuxDmabufFeedbackV1 {
            const INTERFACE: &'static str = "zwp_linux_dmabuf_feedback_v1";
            const VERSION: u32 = 5;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("zwp_linux_dmabuf_feedback_v1 -> destroy");
                        Self::r#destroy(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Using this request a client can tell the server that it is not going to"#]
            #[doc = r#"use the wp_linux_dmabuf_feedback object anymore."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"This event is sent after all parameters of a wp_linux_dmabuf_feedback"#]
            #[doc = r#"object have been sent."#]
            #[doc = r#""#]
            #[doc = r#"This allows changes to the wp_linux_dmabuf_feedback parameters to be"#]
            #[doc = r#"seen as atomic, even if they happen via multiple events."#]
            async fn r#done(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_linux_dmabuf_feedback_v1 -> done");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event provides a file descriptor which can be memory-mapped to"#]
            #[doc = r#"access the format and modifier table."#]
            #[doc = r#""#]
            #[doc = r#"The table contains a tightly packed array of consecutive format +"#]
            #[doc = r#"modifier pairs. Each pair is 16 bytes wide. It contains a format as a"#]
            #[doc = r#"32-bit unsigned integer, followed by 4 bytes of unused padding, and a"#]
            #[doc = r#"modifier as a 64-bit unsigned integer. The native endianness is used."#]
            #[doc = r#""#]
            #[doc = r#"The client must map the file descriptor in read-only private mode."#]
            #[doc = r#""#]
            #[doc = r#"Compositors are not allowed to mutate the table file contents once this"#]
            #[doc = r#"event has been sent. Instead, compositors must create a new, separate"#]
            #[doc = r#"table file and re-send feedback parameters. Compositors are allowed to"#]
            #[doc = r#"store duplicate format + modifier pairs in the table."#]
            async fn r#format_table(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#fd: std::os::fd::RawFd,
                r#size: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_linux_dmabuf_feedback_v1 -> format_table");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_int(fd)
                    .put_uint(size)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event advertises the main device that the server prefers to use"#]
            #[doc = r#"when direct scan-out to the target device isn't possible. The"#]
            #[doc = r#"advertised main device may be different for each"#]
            #[doc = r#"wp_linux_dmabuf_feedback object, and may change over time."#]
            #[doc = r#""#]
            #[doc = r#"There is exactly one main device. The compositor must send at least"#]
            #[doc = r#"one preference tranche with tranche_target_device equal to main_device."#]
            #[doc = r#""#]
            #[doc = r#"Clients need to create buffers that the main device can import and"#]
            #[doc = r#"read from, otherwise creating the dmabuf wl_buffer will fail (see the"#]
            #[doc = r#"wp_linux_buffer_params.create and create_immed requests for details)."#]
            #[doc = r#"The main device will also likely be kept active by the compositor,"#]
            #[doc = r#"so clients can use it instead of waking up another device for power"#]
            #[doc = r#"savings."#]
            #[doc = r#""#]
            #[doc = r#"In general the device is a DRM node. The DRM node type (primary vs."#]
            #[doc = r#"render) is unspecified. Clients must not rely on the compositor sending"#]
            #[doc = r#"a particular node type. Clients cannot check two devices for equality"#]
            #[doc = r#"by comparing the dev_t value."#]
            #[doc = r#""#]
            #[doc = r#"If explicit modifiers are not supported and the client performs buffer"#]
            #[doc = r#"allocations on a different device than the main device, then the client"#]
            #[doc = r#"must force the buffer to have a linear layout."#]
            async fn r#main_device(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#device: Vec<u8>,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_linux_dmabuf_feedback_v1 -> main_device");
                let payload = crate::wire::PayloadBuilder::new().put_array(device).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event splits tranche_target_device and tranche_formats events in"#]
            #[doc = r#"preference tranches. It is sent after a set of tranche_target_device"#]
            #[doc = r#"and tranche_formats events; it represents the end of a tranche. The"#]
            #[doc = r#"next tranche will have a lower preference."#]
            async fn r#tranche_done(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_linux_dmabuf_feedback_v1 -> tranche_done");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 3, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event advertises the target device that the server prefers to use"#]
            #[doc = r#"for a buffer created given this tranche. The advertised target device"#]
            #[doc = r#"may be different for each preference tranche, and may change over time."#]
            #[doc = r#""#]
            #[doc = r#"There is exactly one target device per tranche."#]
            #[doc = r#""#]
            #[doc = r#"The target device may be a scan-out device, for example if the"#]
            #[doc = r#"compositor prefers to directly scan-out a buffer created given this"#]
            #[doc = r#"tranche. The target device may be a rendering device, for example if"#]
            #[doc = r#"the compositor prefers to texture from said buffer."#]
            #[doc = r#""#]
            #[doc = r#"The client can use this hint to allocate the buffer in a way that makes"#]
            #[doc = r#"it accessible from the target device, ideally directly. The buffer must"#]
            #[doc = r#"still be accessible from the main device, either through direct import"#]
            #[doc = r#"or through a potentially more expensive fallback path. If the buffer"#]
            #[doc = r#"can't be directly imported from the main device then clients must be"#]
            #[doc = r#"prepared for the compositor changing the tranche priority or making"#]
            #[doc = r#"wl_buffer creation fail (see the wp_linux_buffer_params.create and"#]
            #[doc = r#"create_immed requests for details)."#]
            #[doc = r#""#]
            #[doc = r#"If the device is a DRM node, the DRM node type (primary vs. render) is"#]
            #[doc = r#"unspecified. Clients must not rely on the compositor sending a"#]
            #[doc = r#"particular node type. Clients cannot check two devices for equality by"#]
            #[doc = r#"comparing the dev_t value."#]
            #[doc = r#""#]
            #[doc = r#"This event is tied to a preference tranche, see the tranche_done event."#]
            async fn r#tranche_target_device(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#device: Vec<u8>,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_linux_dmabuf_feedback_v1 -> tranche_target_device");
                let payload = crate::wire::PayloadBuilder::new().put_array(device).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 4, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event advertises the format + modifier combinations that the"#]
            #[doc = r#"compositor supports."#]
            #[doc = r#""#]
            #[doc = r#"It carries an array of indices, each referring to a format + modifier"#]
            #[doc = r#"pair in the last received format table (see the format_table event)."#]
            #[doc = r#"Each index is a 16-bit unsigned integer in native endianness."#]
            #[doc = r#""#]
            #[doc = r#"For legacy support, DRM_FORMAT_MOD_INVALID is an allowed modifier."#]
            #[doc = r#"It indicates that the server can support the format with an implicit"#]
            #[doc = r#"modifier. When a buffer has DRM_FORMAT_MOD_INVALID as its modifier, it"#]
            #[doc = r#"is as if no explicit modifier is specified. The effective modifier"#]
            #[doc = r#"will be derived from the dmabuf."#]
            #[doc = r#""#]
            #[doc = r#"A compositor that sends valid modifiers and DRM_FORMAT_MOD_INVALID for"#]
            #[doc = r#"a given format supports both explicit modifiers and implicit modifiers."#]
            #[doc = r#""#]
            #[doc = r#"Compositors must not send duplicate format + modifier pairs within the"#]
            #[doc = r#"same tranche or across two different tranches with the same target"#]
            #[doc = r#"device and flags."#]
            #[doc = r#""#]
            #[doc = r#"This event is tied to a preference tranche, see the tranche_done event."#]
            #[doc = r#""#]
            #[doc = r#"For the definition of the format and modifier codes, see the"#]
            #[doc = r#"wp_linux_buffer_params.create request."#]
            async fn r#tranche_formats(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#indices: Vec<u8>,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_linux_dmabuf_feedback_v1 -> tranche_formats");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_array(indices)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 5, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event sets tranche-specific flags."#]
            #[doc = r#""#]
            #[doc = r#"The scanout flag is a hint that direct scan-out may be attempted by the"#]
            #[doc = r#"compositor on the target device if the client appropriately allocates a"#]
            #[doc = r#"buffer. How to allocate a buffer that can be scanned out on the target"#]
            #[doc = r#"device is implementation-defined."#]
            #[doc = r#""#]
            #[doc = r#"This event is tied to a preference tranche, see the tranche_done event."#]
            async fn r#tranche_flags(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#flags: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_linux_dmabuf_feedback_v1 -> tranche_flags");
                let payload = crate::wire::PayloadBuilder::new().put_uint(flags).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 6, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
}
pub mod presentation_time {
    pub mod wp_presentation {
        enum r#Error {}
        #[doc = r#"The main feature of this interface is accurate presentation"#]
        #[doc = r#"timing feedback to ensure smooth video playback while maintaining"#]
        #[doc = r#"audio/video synchronization. Some features use the concept of a"#]
        #[doc = r#"presentation clock, which is defined in the"#]
        #[doc = r#"presentation.clock_id event."#]
        #[doc = r#""#]
        #[doc = r#"A content update for a wl_surface is submitted by a"#]
        #[doc = r#"wl_surface.commit request. Request 'feedback' associates with"#]
        #[doc = r#"the wl_surface.commit and provides feedback on the content"#]
        #[doc = r#"update, particularly the final realized presentation time."#]
        #[doc = r#""#]
        #[doc = r#""#]
        #[doc = r#""#]
        #[doc = r#"When the final realized presentation time is available, e.g."#]
        #[doc = r#"after a framebuffer flip completes, the requested"#]
        #[doc = r#"presentation_feedback.presented events are sent. The final"#]
        #[doc = r#"presentation time can differ from the compositor's predicted"#]
        #[doc = r#"display update time and the update's target time, especially"#]
        #[doc = r#"when the compositor misses its target vertical blanking period."#]
        pub trait r#WpPresentation {
            const INTERFACE: &'static str = "wp_presentation";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wp_presentation -> destroy");
                        Self::r#destroy(client).await
                    }
                    1 => {
                        tracing::debug!("wp_presentation -> feedback");
                        Self::r#feedback(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Informs the server that the client will no longer be using"#]
            #[doc = r#"this protocol object. Existing objects created by this object"#]
            #[doc = r#"are not affected."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Request presentation feedback for the current content submission"#]
            #[doc = r#"on the given surface. This creates a new presentation_feedback"#]
            #[doc = r#"object, which will deliver the feedback information once. If"#]
            #[doc = r#"multiple presentation_feedback objects are created for the same"#]
            #[doc = r#"submission, they will all deliver the same information."#]
            #[doc = r#""#]
            #[doc = r#"For details on what information is returned, see the"#]
            #[doc = r#"presentation_feedback interface."#]
            async fn r#feedback(
                client: &mut crate::Client,
                r#surface: crate::wire::ObjectId,
                r#callback: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"This event tells the client in which clock domain the"#]
            #[doc = r#"compositor interprets the timestamps used by the presentation"#]
            #[doc = r#"extension. This clock is called the presentation clock."#]
            #[doc = r#""#]
            #[doc = r#"The compositor sends this event when the client binds to the"#]
            #[doc = r#"presentation interface. The presentation clock does not change"#]
            #[doc = r#"during the lifetime of the client connection."#]
            #[doc = r#""#]
            #[doc = r#"The clock identifier is platform dependent. On POSIX platforms, the"#]
            #[doc = r#"identifier value is one of the clockid_t values accepted by"#]
            #[doc = r#"clock_gettime(). clock_gettime() is defined by POSIX.1-2001."#]
            #[doc = r#""#]
            #[doc = r#"Timestamps in this clock domain are expressed as tv_sec_hi,"#]
            #[doc = r#"tv_sec_lo, tv_nsec triples, each component being an unsigned"#]
            #[doc = r#"32-bit value. Whole seconds are in tv_sec which is a 64-bit"#]
            #[doc = r#"value combined from tv_sec_hi and tv_sec_lo, and the"#]
            #[doc = r#"additional fractional part in tv_nsec as nanoseconds. Hence,"#]
            #[doc = r#"for valid timestamps tv_nsec must be in [0, 999999999]."#]
            #[doc = r#""#]
            #[doc = r#"Note that clock_id applies only to the presentation clock,"#]
            #[doc = r#"and implies nothing about e.g. the timestamps used in the"#]
            #[doc = r#"Wayland core protocol input events."#]
            #[doc = r#""#]
            #[doc = r#"Compositors should prefer a clock which does not jump and is"#]
            #[doc = r#"not slewed e.g. by NTP. The absolute value of the clock is"#]
            #[doc = r#"irrelevant. Precision of one millisecond or better is"#]
            #[doc = r#"recommended. Clients must be able to query the current clock"#]
            #[doc = r#"value directly, not by asking the compositor."#]
            async fn r#clock_id(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#clk_id: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wp_presentation -> clock_id");
                let payload = crate::wire::PayloadBuilder::new().put_uint(clk_id).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod wp_presentation_feedback {
        enum r#Kind {}
        #[doc = r#"A presentation_feedback object returns an indication that a"#]
        #[doc = r#"wl_surface content update has become visible to the user."#]
        #[doc = r#"One object corresponds to one content update submission"#]
        #[doc = r#"(wl_surface.commit). There are two possible outcomes: the"#]
        #[doc = r#"content update is presented to the user, and a presentation"#]
        #[doc = r#"timestamp delivered; or, the user did not see the content"#]
        #[doc = r#"update because it was superseded or its surface destroyed,"#]
        #[doc = r#"and the content update is discarded."#]
        #[doc = r#""#]
        #[doc = r#"Once a presentation_feedback object has delivered a 'presented'"#]
        #[doc = r#"or 'discarded' event it is automatically destroyed."#]
        pub trait r#WpPresentationFeedback {
            const INTERFACE: &'static str = "wp_presentation_feedback";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"As presentation can be synchronized to only one output at a"#]
            #[doc = r#"time, this event tells which output it was. This event is only"#]
            #[doc = r#"sent prior to the presented event."#]
            #[doc = r#""#]
            #[doc = r#"As clients may bind to the same global wl_output multiple"#]
            #[doc = r#"times, this event is sent for each bound instance that matches"#]
            #[doc = r#"the synchronized output. If a client has not bound to the"#]
            #[doc = r#"right wl_output global at all, this event is not sent."#]
            async fn r#sync_output(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#output: crate::wire::ObjectId,
            ) -> crate::Result<()> {
                tracing::debug!("wp_presentation_feedback -> sync_output");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_object(Some(output))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"The associated content update was displayed to the user at the"#]
            #[doc = r#"indicated time (tv_sec_hi/lo, tv_nsec). For the interpretation of"#]
            #[doc = r#"the timestamp, see presentation.clock_id event."#]
            #[doc = r#""#]
            #[doc = r#"The timestamp corresponds to the time when the content update"#]
            #[doc = r#"turned into light the first time on the surface's main output."#]
            #[doc = r#"Compositors may approximate this from the framebuffer flip"#]
            #[doc = r#"completion events from the system, and the latency of the"#]
            #[doc = r#"physical display path if known."#]
            #[doc = r#""#]
            #[doc = r#"This event is preceded by all related sync_output events"#]
            #[doc = r#"telling which output's refresh cycle the feedback corresponds"#]
            #[doc = r#"to, i.e. the main output for the surface. Compositors are"#]
            #[doc = r#"recommended to choose the output containing the largest part"#]
            #[doc = r#"of the wl_surface, or keeping the output they previously"#]
            #[doc = r#"chose. Having a stable presentation output association helps"#]
            #[doc = r#"clients predict future output refreshes (vblank)."#]
            #[doc = r#""#]
            #[doc = r#"The 'refresh' argument gives the compositor's prediction of how"#]
            #[doc = r#"many nanoseconds after tv_sec, tv_nsec the very next output"#]
            #[doc = r#"refresh may occur. This is to further aid clients in"#]
            #[doc = r#"predicting future refreshes, i.e., estimating the timestamps"#]
            #[doc = r#"targeting the next few vblanks. If such prediction cannot"#]
            #[doc = r#"usefully be done, the argument is zero."#]
            #[doc = r#""#]
            #[doc = r#"If the output does not have a constant refresh rate, explicit"#]
            #[doc = r#"video mode switches excluded, then the refresh argument must"#]
            #[doc = r#"be zero."#]
            #[doc = r#""#]
            #[doc = r#"The 64-bit value combined from seq_hi and seq_lo is the value"#]
            #[doc = r#"of the output's vertical retrace counter when the content"#]
            #[doc = r#"update was first scanned out to the display. This value must"#]
            #[doc = r#"be compatible with the definition of MSC in"#]
            #[doc = r#"GLX_OML_sync_control specification. Note, that if the display"#]
            #[doc = r#"path has a non-zero latency, the time instant specified by"#]
            #[doc = r#"this counter may differ from the timestamp's."#]
            #[doc = r#""#]
            #[doc = r#"If the output does not have a concept of vertical retrace or a"#]
            #[doc = r#"refresh cycle, or the output device is self-refreshing without"#]
            #[doc = r#"a way to query the refresh count, then the arguments seq_hi"#]
            #[doc = r#"and seq_lo must be zero."#]
            async fn r#presented(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#tv_sec_hi: u32,
                r#tv_sec_lo: u32,
                r#tv_nsec: u32,
                r#refresh: u32,
                r#seq_hi: u32,
                r#seq_lo: u32,
                r#flags: u32,
            ) -> crate::Result<()> {
                tracing::debug!("wp_presentation_feedback -> presented");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(tv_sec_hi)
                    .put_uint(tv_sec_lo)
                    .put_uint(tv_nsec)
                    .put_uint(refresh)
                    .put_uint(seq_hi)
                    .put_uint(seq_lo)
                    .put_uint(flags)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"The content update was never displayed to the user."#]
            async fn r#discarded(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("wp_presentation_feedback -> discarded");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
}
#[doc = r#"This description provides a high-level overview of the interplay between"#]
#[doc = r#"the interfaces defined this protocol. For details, see the protocol"#]
#[doc = r#"specification."#]
#[doc = r#""#]
#[doc = r#"More than one tablet may exist, and device-specifics matter. Tablets are"#]
#[doc = r#"not represented by a single virtual device like wl_pointer. A client"#]
#[doc = r#"binds to the tablet manager object which is just a proxy object. From"#]
#[doc = r#"that, the client requests wp_tablet_manager.get_tablet_seat(wl_seat)"#]
#[doc = r#"and that returns the actual interface that has all the tablets. With"#]
#[doc = r#"this indirection, we can avoid merging wp_tablet into the actual Wayland"#]
#[doc = r#"protocol, a long-term benefit."#]
#[doc = r#""#]
#[doc = r#"The wp_tablet_seat sends a "tablet added" event for each tablet"#]
#[doc = r#"connected. That event is followed by descriptive events about the"#]
#[doc = r#"hardware; currently that includes events for name, vid/pid and"#]
#[doc = r#"a wp_tablet.path event that describes a local path. This path can be"#]
#[doc = r#"used to uniquely identify a tablet or get more information through"#]
#[doc = r#"libwacom. Emulated or nested tablets can skip any of those, e.g. a"#]
#[doc = r#"virtual tablet may not have a vid/pid. The sequence of descriptive"#]
#[doc = r#"events is terminated by a wp_tablet.done event to signal that a client"#]
#[doc = r#"may now finalize any initialization for that tablet."#]
#[doc = r#""#]
#[doc = r#"Events from tablets require a tool in proximity. Tools are also managed"#]
#[doc = r#"by the tablet seat; a "tool added" event is sent whenever a tool is new"#]
#[doc = r#"to the compositor. That event is followed by a number of descriptive"#]
#[doc = r#"events about the hardware; currently that includes capabilities,"#]
#[doc = r#"hardware id and serial number, and tool type. Similar to the tablet"#]
#[doc = r#"interface, a wp_tablet_tool.done event is sent to terminate that initial"#]
#[doc = r#"sequence."#]
#[doc = r#""#]
#[doc = r#"Any event from a tool happens on the wp_tablet_tool interface. When the"#]
#[doc = r#"tool gets into proximity of the tablet, a proximity_in event is sent on"#]
#[doc = r#"the wp_tablet_tool interface, listing the tablet and the surface. That"#]
#[doc = r#"event is followed by a motion event with the coordinates. After that,"#]
#[doc = r#"it's the usual motion, axis, button, etc. events. The protocol's"#]
#[doc = r#"serialisation means events are grouped by wp_tablet_tool.frame events."#]
#[doc = r#""#]
#[doc = r#"Two special events (that don't exist in X) are down and up. They signal"#]
#[doc = r#""tip touching the surface". For tablets without real proximity"#]
#[doc = r#"detection, the sequence is: proximity_in, motion, down, frame."#]
#[doc = r#""#]
#[doc = r#"When the tool leaves proximity, a proximity_out event is sent. If any"#]
#[doc = r#"button is still down, a button release event is sent before this"#]
#[doc = r#"proximity event. These button events are sent in the same frame as the"#]
#[doc = r#"proximity event to signal to the client that the buttons were held when"#]
#[doc = r#"the tool left proximity."#]
#[doc = r#""#]
#[doc = r#"If the tool moves out of the surface but stays in proximity (i.e."#]
#[doc = r#"between windows), compositor-specific grab policies apply. This usually"#]
#[doc = r#"means that the proximity-out is delayed until all buttons are released."#]
#[doc = r#""#]
#[doc = r#"Moving a tool physically from one tablet to the other has no real effect"#]
#[doc = r#"on the protocol, since we already have the tool object from the "tool"#]
#[doc = r#"added" event. All the information is already there and the proximity"#]
#[doc = r#"events on both tablets are all a client needs to reconstruct what"#]
#[doc = r#"happened."#]
#[doc = r#""#]
#[doc = r#"Some extra axes are normalized, i.e. the client knows the range as"#]
#[doc = r#"specified in the protocol (e.g. [0, 65535]), the granularity however is"#]
#[doc = r#"unknown. The current normalized axes are pressure, distance, and slider."#]
#[doc = r#""#]
#[doc = r#"Other extra axes are in physical units as specified in the protocol."#]
#[doc = r#"The current extra axes with physical units are tilt, rotation and"#]
#[doc = r#"wheel rotation."#]
#[doc = r#""#]
#[doc = r#"Since tablets work independently of the pointer controlled by the mouse,"#]
#[doc = r#"the focus handling is independent too and controlled by proximity."#]
#[doc = r#"The wp_tablet_tool.set_cursor request sets a tool-specific cursor."#]
#[doc = r#"This cursor surface may be the same as the mouse cursor, and it may be"#]
#[doc = r#"the same across tools but it is possible to be more fine-grained. For"#]
#[doc = r#"example, a client may set different cursors for the pen and eraser."#]
#[doc = r#""#]
#[doc = r#"Tools are generally independent of tablets and it is"#]
#[doc = r#"compositor-specific policy when a tool can be removed. Common approaches"#]
#[doc = r#"will likely include some form of removing a tool when all tablets the"#]
#[doc = r#"tool was used on are removed."#]
pub mod tablet_v2 {
    pub mod zwp_tablet_manager_v2 {
        #[doc = r#"An object that provides access to the graphics tablets available on this"#]
        #[doc = r#"system. All tablets are associated with a seat, to get access to the"#]
        #[doc = r#"actual tablets, use wp_tablet_manager.get_tablet_seat."#]
        pub trait r#ZwpTabletManagerV2 {
            const INTERFACE: &'static str = "zwp_tablet_manager_v2";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("zwp_tablet_manager_v2 -> get_tablet_seat");
                        Self::r#get_tablet_seat(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    1 => {
                        tracing::debug!("zwp_tablet_manager_v2 -> destroy");
                        Self::r#destroy(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Get the wp_tablet_seat object for the given seat. This object"#]
            #[doc = r#"provides access to all graphics tablets in this seat."#]
            async fn r#get_tablet_seat(
                client: &mut crate::Client,
                r#tablet_seat: crate::wire::ObjectId,
                r#seat: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"Destroy the wp_tablet_manager object. Objects created from this"#]
            #[doc = r#"object are unaffected and should be destroyed separately."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
        }
    }
    pub mod zwp_tablet_seat_v2 {
        #[doc = r#"An object that provides access to the graphics tablets available on this"#]
        #[doc = r#"seat. After binding to this interface, the compositor sends a set of"#]
        #[doc = r#"wp_tablet_seat.tablet_added and wp_tablet_seat.tool_added events."#]
        pub trait r#ZwpTabletSeatV2 {
            const INTERFACE: &'static str = "zwp_tablet_seat_v2";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("zwp_tablet_seat_v2 -> destroy");
                        Self::r#destroy(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Destroy the wp_tablet_seat object. Objects created from this"#]
            #[doc = r#"object are unaffected and should be destroyed separately."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"This event is sent whenever a new tablet becomes available on this"#]
            #[doc = r#"seat. This event only provides the object id of the tablet, any"#]
            #[doc = r#"static information about the tablet (device name, vid/pid, etc.) is"#]
            #[doc = r#"sent through the wp_tablet interface."#]
            async fn r#tablet_added(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_seat_v2 -> tablet_added");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event is sent whenever a tool that has not previously been used"#]
            #[doc = r#"with a tablet comes into use. This event only provides the object id"#]
            #[doc = r#"of the tool; any static information about the tool (capabilities,"#]
            #[doc = r#"type, etc.) is sent through the wp_tablet_tool interface."#]
            async fn r#tool_added(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_seat_v2 -> tool_added");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event is sent whenever a new pad is known to the system. Typically,"#]
            #[doc = r#"pads are physically attached to tablets and a pad_added event is"#]
            #[doc = r#"sent immediately after the wp_tablet_seat.tablet_added."#]
            #[doc = r#"However, some standalone pad devices logically attach to tablets at"#]
            #[doc = r#"runtime, and the client must wait for wp_tablet_pad.enter to know"#]
            #[doc = r#"the tablet a pad is attached to."#]
            #[doc = r#""#]
            #[doc = r#"This event only provides the object id of the pad. All further"#]
            #[doc = r#"features (buttons, strips, rings) are sent through the wp_tablet_pad"#]
            #[doc = r#"interface."#]
            async fn r#pad_added(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_seat_v2 -> pad_added");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_object(Some(id))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod zwp_tablet_tool_v2 {
        enum r#Type {}
        enum r#Capability {}
        enum r#ButtonState {}
        enum r#Error {}
        #[doc = r#"An object that represents a physical tool that has been, or is"#]
        #[doc = r#"currently in use with a tablet in this seat. Each wp_tablet_tool"#]
        #[doc = r#"object stays valid until the client destroys it; the compositor"#]
        #[doc = r#"reuses the wp_tablet_tool object to indicate that the object's"#]
        #[doc = r#"respective physical tool has come into proximity of a tablet again."#]
        #[doc = r#""#]
        #[doc = r#"A wp_tablet_tool object's relation to a physical tool depends on the"#]
        #[doc = r#"tablet's ability to report serial numbers. If the tablet supports"#]
        #[doc = r#"this capability, then the object represents a specific physical tool"#]
        #[doc = r#"and can be identified even when used on multiple tablets."#]
        #[doc = r#""#]
        #[doc = r#"A tablet tool has a number of static characteristics, e.g. tool type,"#]
        #[doc = r#"hardware_serial and capabilities. These capabilities are sent in an"#]
        #[doc = r#"event sequence after the wp_tablet_seat.tool_added event before any"#]
        #[doc = r#"actual events from this tool. This initial event sequence is"#]
        #[doc = r#"terminated by a wp_tablet_tool.done event."#]
        #[doc = r#""#]
        #[doc = r#"Tablet tool events are grouped by wp_tablet_tool.frame events."#]
        #[doc = r#"Any events received before a wp_tablet_tool.frame event should be"#]
        #[doc = r#"considered part of the same hardware state change."#]
        pub trait r#ZwpTabletToolV2 {
            const INTERFACE: &'static str = "zwp_tablet_tool_v2";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("zwp_tablet_tool_v2 -> set_cursor");
                        Self::r#set_cursor(
                            client,
                            message.uint()?,
                            message.object()?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    1 => {
                        tracing::debug!("zwp_tablet_tool_v2 -> destroy");
                        Self::r#destroy(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Sets the surface of the cursor used for this tool on the given"#]
            #[doc = r#"tablet. This request only takes effect if the tool is in proximity"#]
            #[doc = r#"of one of the requesting client's surfaces or the surface parameter"#]
            #[doc = r#"is the current pointer surface. If there was a previous surface set"#]
            #[doc = r#"with this request it is replaced. If surface is NULL, the cursor"#]
            #[doc = r#"image is hidden."#]
            #[doc = r#""#]
            #[doc = r#"The parameters hotspot_x and hotspot_y define the position of the"#]
            #[doc = r#"pointer surface relative to the pointer location. Its top-left corner"#]
            #[doc = r#"is always at (x, y) - (hotspot_x, hotspot_y), where (x, y) are the"#]
            #[doc = r#"coordinates of the pointer location, in surface-local coordinates."#]
            #[doc = r#""#]
            #[doc = r#"On surface.attach requests to the pointer surface, hotspot_x and"#]
            #[doc = r#"hotspot_y are decremented by the x and y parameters passed to the"#]
            #[doc = r#"request. Attach must be confirmed by wl_surface.commit as usual."#]
            #[doc = r#""#]
            #[doc = r#"The hotspot can also be updated by passing the currently set pointer"#]
            #[doc = r#"surface to this request with new values for hotspot_x and hotspot_y."#]
            #[doc = r#""#]
            #[doc = r#"The current and pending input regions of the wl_surface are cleared,"#]
            #[doc = r#"and wl_surface.set_input_region is ignored until the wl_surface is no"#]
            #[doc = r#"longer used as the cursor. When the use as a cursor ends, the current"#]
            #[doc = r#"and pending input regions become undefined, and the wl_surface is"#]
            #[doc = r#"unmapped."#]
            #[doc = r#""#]
            #[doc = r#"This request gives the surface the role of a wp_tablet_tool cursor. A"#]
            #[doc = r#"surface may only ever be used as the cursor surface for one"#]
            #[doc = r#"wp_tablet_tool. If the surface already has another role or has"#]
            #[doc = r#"previously been used as cursor surface for a different tool, a"#]
            #[doc = r#"protocol error is raised."#]
            async fn r#set_cursor(
                client: &mut crate::Client,
                r#serial: u32,
                r#surface: Option<crate::wire::ObjectId>,
                r#hotspot_x: i32,
                r#hotspot_y: i32,
            ) -> crate::Result<()>;
            #[doc = r#"This destroys the client's resource for this tool object."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"The tool type is the high-level type of the tool and usually decides"#]
            #[doc = r#"the interaction expected from this tool."#]
            #[doc = r#""#]
            #[doc = r#"This event is sent in the initial burst of events before the"#]
            #[doc = r#"wp_tablet_tool.done event."#]
            async fn r#type(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#tool_type: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> type");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(tool_type)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"If the physical tool can be identified by a unique 64-bit serial"#]
            #[doc = r#"number, this event notifies the client of this serial number."#]
            #[doc = r#""#]
            #[doc = r#"If multiple tablets are available in the same seat and the tool is"#]
            #[doc = r#"uniquely identifiable by the serial number, that tool may move"#]
            #[doc = r#"between tablets."#]
            #[doc = r#""#]
            #[doc = r#"Otherwise, if the tool has no serial number and this event is"#]
            #[doc = r#"missing, the tool is tied to the tablet it first comes into"#]
            #[doc = r#"proximity with. Even if the physical tool is used on multiple"#]
            #[doc = r#"tablets, separate wp_tablet_tool objects will be created, one per"#]
            #[doc = r#"tablet."#]
            #[doc = r#""#]
            #[doc = r#"This event is sent in the initial burst of events before the"#]
            #[doc = r#"wp_tablet_tool.done event."#]
            async fn r#hardware_serial(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#hardware_serial_hi: u32,
                r#hardware_serial_lo: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> hardware_serial");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(hardware_serial_hi)
                    .put_uint(hardware_serial_lo)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event notifies the client of a hardware id available on this tool."#]
            #[doc = r#""#]
            #[doc = r#"The hardware id is a device-specific 64-bit id that provides extra"#]
            #[doc = r#"information about the tool in use, beyond the wl_tool.type"#]
            #[doc = r#"enumeration. The format of the id is specific to tablets made by"#]
            #[doc = r#"Wacom Inc. For example, the hardware id of a Wacom Grip"#]
            #[doc = r#"Pen (a stylus) is 0x802."#]
            #[doc = r#""#]
            #[doc = r#"This event is sent in the initial burst of events before the"#]
            #[doc = r#"wp_tablet_tool.done event."#]
            async fn r#hardware_id_wacom(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#hardware_id_hi: u32,
                r#hardware_id_lo: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> hardware_id_wacom");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(hardware_id_hi)
                    .put_uint(hardware_id_lo)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event notifies the client of any capabilities of this tool,"#]
            #[doc = r#"beyond the main set of x/y axes and tip up/down detection."#]
            #[doc = r#""#]
            #[doc = r#"One event is sent for each extra capability available on this tool."#]
            #[doc = r#""#]
            #[doc = r#"This event is sent in the initial burst of events before the"#]
            #[doc = r#"wp_tablet_tool.done event."#]
            async fn r#capability(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#capability: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> capability");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(capability)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 3, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event signals the end of the initial burst of descriptive"#]
            #[doc = r#"events. A client may consider the static description of the tool to"#]
            #[doc = r#"be complete and finalize initialization of the tool."#]
            async fn r#done(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> done");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 4, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event is sent when the tool is removed from the system and will"#]
            #[doc = r#"send no further events. Should the physical tool come back into"#]
            #[doc = r#"proximity later, a new wp_tablet_tool object will be created."#]
            #[doc = r#""#]
            #[doc = r#"It is compositor-dependent when a tool is removed. A compositor may"#]
            #[doc = r#"remove a tool on proximity out, tablet removal or any other reason."#]
            #[doc = r#"A compositor may also keep a tool alive until shutdown."#]
            #[doc = r#""#]
            #[doc = r#"If the tool is currently in proximity, a proximity_out event will be"#]
            #[doc = r#"sent before the removed event. See wp_tablet_tool.proximity_out for"#]
            #[doc = r#"the handling of any buttons logically down."#]
            #[doc = r#""#]
            #[doc = r#"When this event is received, the client must wp_tablet_tool.destroy"#]
            #[doc = r#"the object."#]
            async fn r#removed(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> removed");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 5, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Notification that this tool is focused on a certain surface."#]
            #[doc = r#""#]
            #[doc = r#"This event can be received when the tool has moved from one surface to"#]
            #[doc = r#"another, or when the tool has come back into proximity above the"#]
            #[doc = r#"surface."#]
            #[doc = r#""#]
            #[doc = r#"If any button is logically down when the tool comes into proximity,"#]
            #[doc = r#"the respective button event is sent after the proximity_in event but"#]
            #[doc = r#"within the same frame as the proximity_in event."#]
            async fn r#proximity_in(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
                r#tablet: crate::wire::ObjectId,
                r#surface: crate::wire::ObjectId,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> proximity_in");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(Some(tablet))
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 6, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Notification that this tool has either left proximity, or is no"#]
            #[doc = r#"longer focused on a certain surface."#]
            #[doc = r#""#]
            #[doc = r#"When the tablet tool leaves proximity of the tablet, button release"#]
            #[doc = r#"events are sent for each button that was held down at the time of"#]
            #[doc = r#"leaving proximity. These events are sent before the proximity_out"#]
            #[doc = r#"event but within the same wp_tablet.frame."#]
            #[doc = r#""#]
            #[doc = r#"If the tool stays within proximity of the tablet, but the focus"#]
            #[doc = r#"changes from one surface to another, a button release event may not"#]
            #[doc = r#"be sent until the button is actually released or the tool leaves the"#]
            #[doc = r#"proximity of the tablet."#]
            async fn r#proximity_out(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> proximity_out");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 7, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent whenever the tablet tool comes in contact with the surface of the"#]
            #[doc = r#"tablet."#]
            #[doc = r#""#]
            #[doc = r#"If the tool is already in contact with the tablet when entering the"#]
            #[doc = r#"input region, the client owning said region will receive a"#]
            #[doc = r#"wp_tablet.proximity_in event, followed by a wp_tablet.down"#]
            #[doc = r#"event and a wp_tablet.frame event."#]
            #[doc = r#""#]
            #[doc = r#"Note that this event describes logical contact, not physical"#]
            #[doc = r#"contact. On some devices, a compositor may not consider a tool in"#]
            #[doc = r#"logical contact until a minimum physical pressure threshold is"#]
            #[doc = r#"exceeded."#]
            async fn r#down(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> down");
                let payload = crate::wire::PayloadBuilder::new().put_uint(serial).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 8, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent whenever the tablet tool stops making contact with the surface of"#]
            #[doc = r#"the tablet, or when the tablet tool moves out of the input region"#]
            #[doc = r#"and the compositor grab (if any) is dismissed."#]
            #[doc = r#""#]
            #[doc = r#"If the tablet tool moves out of the input region while in contact"#]
            #[doc = r#"with the surface of the tablet and the compositor does not have an"#]
            #[doc = r#"ongoing grab on the surface, the client owning said region will"#]
            #[doc = r#"receive a wp_tablet.up event, followed by a wp_tablet.proximity_out"#]
            #[doc = r#"event and a wp_tablet.frame event. If the compositor has an ongoing"#]
            #[doc = r#"grab on this device, this event sequence is sent whenever the grab"#]
            #[doc = r#"is dismissed in the future."#]
            #[doc = r#""#]
            #[doc = r#"Note that this event describes logical contact, not physical"#]
            #[doc = r#"contact. On some devices, a compositor may not consider a tool out"#]
            #[doc = r#"of logical contact until physical pressure falls below a specific"#]
            #[doc = r#"threshold."#]
            async fn r#up(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> up");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 9, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent whenever a tablet tool moves."#]
            async fn r#motion(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#x: crate::wire::Fixed,
                r#y: crate::wire::Fixed,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> motion");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_fixed(x)
                    .put_fixed(y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 10, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent whenever the pressure axis on a tool changes. The value of this"#]
            #[doc = r#"event is normalized to a value between 0 and 65535."#]
            #[doc = r#""#]
            #[doc = r#"Note that pressure may be nonzero even when a tool is not in logical"#]
            #[doc = r#"contact. See the down and up events for more details."#]
            async fn r#pressure(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#pressure: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> pressure");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(pressure)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 11, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent whenever the distance axis on a tool changes. The value of this"#]
            #[doc = r#"event is normalized to a value between 0 and 65535."#]
            #[doc = r#""#]
            #[doc = r#"Note that distance may be nonzero even when a tool is not in logical"#]
            #[doc = r#"contact. See the down and up events for more details."#]
            async fn r#distance(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#distance: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> distance");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(distance)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 12, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent whenever one or both of the tilt axes on a tool change. Each tilt"#]
            #[doc = r#"value is in degrees, relative to the z-axis of the tablet."#]
            #[doc = r#"The angle is positive when the top of a tool tilts along the"#]
            #[doc = r#"positive x or y axis."#]
            async fn r#tilt(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#tilt_x: crate::wire::Fixed,
                r#tilt_y: crate::wire::Fixed,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> tilt");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_fixed(tilt_x)
                    .put_fixed(tilt_y)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 13, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent whenever the z-rotation axis on the tool changes. The"#]
            #[doc = r#"rotation value is in degrees clockwise from the tool's"#]
            #[doc = r#"logical neutral position."#]
            async fn r#rotation(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#degrees: crate::wire::Fixed,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> rotation");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_fixed(degrees)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 14, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent whenever the slider position on the tool changes. The"#]
            #[doc = r#"value is normalized between -65535 and 65535, with 0 as the logical"#]
            #[doc = r#"neutral position of the slider."#]
            #[doc = r#""#]
            #[doc = r#"The slider is available on e.g. the Wacom Airbrush tool."#]
            async fn r#slider(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#position: i32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> slider");
                let payload = crate::wire::PayloadBuilder::new().put_int(position).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 15, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent whenever the wheel on the tool emits an event. This event"#]
            #[doc = r#"contains two values for the same axis change. The degrees value is"#]
            #[doc = r#"in the same orientation as the wl_pointer.vertical_scroll axis. The"#]
            #[doc = r#"clicks value is in discrete logical clicks of the mouse wheel. This"#]
            #[doc = r#"value may be zero if the movement of the wheel was less"#]
            #[doc = r#"than one logical click."#]
            #[doc = r#""#]
            #[doc = r#"Clients should choose either value and avoid mixing degrees and"#]
            #[doc = r#"clicks. The compositor may accumulate values smaller than a logical"#]
            #[doc = r#"click and emulate click events when a certain threshold is met."#]
            #[doc = r#"Thus, wl_tablet_tool.wheel events with non-zero clicks values may"#]
            #[doc = r#"have different degrees values."#]
            async fn r#wheel(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#degrees: crate::wire::Fixed,
                r#clicks: i32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> wheel");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_fixed(degrees)
                    .put_int(clicks)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 16, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent whenever a button on the tool is pressed or released."#]
            #[doc = r#""#]
            #[doc = r#"If a button is held down when the tool moves in or out of proximity,"#]
            #[doc = r#"button events are generated by the compositor. See"#]
            #[doc = r#"wp_tablet_tool.proximity_in and wp_tablet_tool.proximity_out for"#]
            #[doc = r#"details."#]
            async fn r#button(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
                r#button: u32,
                r#state: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> button");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_uint(button)
                    .put_uint(state)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 17, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Marks the end of a series of axis and/or button updates from the"#]
            #[doc = r#"tablet. The Wayland protocol requires axis updates to be sent"#]
            #[doc = r#"sequentially, however all events within a frame should be considered"#]
            #[doc = r#"one hardware event."#]
            async fn r#frame(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#time: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_tool_v2 -> frame");
                let payload = crate::wire::PayloadBuilder::new().put_uint(time).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 18, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod zwp_tablet_v2 {
        #[doc = r#"The wp_tablet interface represents one graphics tablet device. The"#]
        #[doc = r#"tablet interface itself does not generate events; all events are"#]
        #[doc = r#"generated by wp_tablet_tool objects when in proximity above a tablet."#]
        #[doc = r#""#]
        #[doc = r#"A tablet has a number of static characteristics, e.g. device name and"#]
        #[doc = r#"pid/vid. These capabilities are sent in an event sequence after the"#]
        #[doc = r#"wp_tablet_seat.tablet_added event. This initial event sequence is"#]
        #[doc = r#"terminated by a wp_tablet.done event."#]
        pub trait r#ZwpTabletV2 {
            const INTERFACE: &'static str = "zwp_tablet_v2";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("zwp_tablet_v2 -> destroy");
                        Self::r#destroy(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"This destroys the client's resource for this tablet object."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"A descriptive name for the tablet device."#]
            #[doc = r#""#]
            #[doc = r#"If the device has no descriptive name, this event is not sent."#]
            #[doc = r#""#]
            #[doc = r#"This event is sent in the initial burst of events before the"#]
            #[doc = r#"wp_tablet.done event."#]
            async fn r#name(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#name: String,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_v2 -> name");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_string(Some(name))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"The USB vendor and product IDs for the tablet device."#]
            #[doc = r#""#]
            #[doc = r#"If the device has no USB vendor/product ID, this event is not sent."#]
            #[doc = r#"This can happen for virtual devices or non-USB devices, for instance."#]
            #[doc = r#""#]
            #[doc = r#"This event is sent in the initial burst of events before the"#]
            #[doc = r#"wp_tablet.done event."#]
            async fn r#id(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#vid: u32,
                r#pid: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_v2 -> id");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(vid)
                    .put_uint(pid)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"A system-specific device path that indicates which device is behind"#]
            #[doc = r#"this wp_tablet. This information may be used to gather additional"#]
            #[doc = r#"information about the device, e.g. through libwacom."#]
            #[doc = r#""#]
            #[doc = r#"A device may have more than one device path. If so, multiple"#]
            #[doc = r#"wp_tablet.path events are sent. A device may be emulated and not"#]
            #[doc = r#"have a device path, and in that case this event will not be sent."#]
            #[doc = r#""#]
            #[doc = r#"The format of the path is unspecified, it may be a device node, a"#]
            #[doc = r#"sysfs path, or some other identifier. It is up to the client to"#]
            #[doc = r#"identify the string provided."#]
            #[doc = r#""#]
            #[doc = r#"This event is sent in the initial burst of events before the"#]
            #[doc = r#"wp_tablet.done event."#]
            async fn r#path(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#path: String,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_v2 -> path");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_string(Some(path))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event is sent immediately to signal the end of the initial"#]
            #[doc = r#"burst of descriptive events. A client may consider the static"#]
            #[doc = r#"description of the tablet to be complete and finalize initialization"#]
            #[doc = r#"of the tablet."#]
            async fn r#done(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_v2 -> done");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 3, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent when the tablet has been removed from the system. When a tablet"#]
            #[doc = r#"is removed, some tools may be removed."#]
            #[doc = r#""#]
            #[doc = r#"When this event is received, the client must wp_tablet.destroy"#]
            #[doc = r#"the object."#]
            async fn r#removed(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_v2 -> removed");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 4, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod zwp_tablet_pad_ring_v2 {
        enum r#Source {}
        #[doc = r#"A circular interaction area, such as the touch ring on the Wacom Intuos"#]
        #[doc = r#"Pro series tablets."#]
        #[doc = r#""#]
        #[doc = r#"Events on a ring are logically grouped by the wl_tablet_pad_ring.frame"#]
        #[doc = r#"event."#]
        pub trait r#ZwpTabletPadRingV2 {
            const INTERFACE: &'static str = "zwp_tablet_pad_ring_v2";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("zwp_tablet_pad_ring_v2 -> set_feedback");
                        Self::r#set_feedback(
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    1 => {
                        tracing::debug!("zwp_tablet_pad_ring_v2 -> destroy");
                        Self::r#destroy(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Request that the compositor use the provided feedback string"#]
            #[doc = r#"associated with this ring. This request should be issued immediately"#]
            #[doc = r#"after a wp_tablet_pad_group.mode_switch event from the corresponding"#]
            #[doc = r#"group is received, or whenever the ring is mapped to a different"#]
            #[doc = r#"action. See wp_tablet_pad_group.mode_switch for more details."#]
            #[doc = r#""#]
            #[doc = r#"Clients are encouraged to provide context-aware descriptions for"#]
            #[doc = r#"the actions associated with the ring; compositors may use this"#]
            #[doc = r#"information to offer visual feedback about the button layout"#]
            #[doc = r#"(eg. on-screen displays)."#]
            #[doc = r#""#]
            #[doc = r#"The provided string 'description' is a UTF-8 encoded string to be"#]
            #[doc = r#"associated with this ring, and is considered user-visible; general"#]
            #[doc = r#"internationalization rules apply."#]
            #[doc = r#""#]
            #[doc = r#"The serial argument will be that of the last"#]
            #[doc = r#"wp_tablet_pad_group.mode_switch event received for the group of this"#]
            #[doc = r#"ring. Requests providing other serials than the most recent one will be"#]
            #[doc = r#"ignored."#]
            async fn r#set_feedback(
                client: &mut crate::Client,
                r#description: String,
                r#serial: u32,
            ) -> crate::Result<()>;
            #[doc = r#"This destroys the client's resource for this ring object."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Source information for ring events."#]
            #[doc = r#""#]
            #[doc = r#"This event does not occur on its own. It is sent before a"#]
            #[doc = r#"wp_tablet_pad_ring.frame event and carries the source information"#]
            #[doc = r#"for all events within that frame."#]
            #[doc = r#""#]
            #[doc = r#"The source specifies how this event was generated. If the source is"#]
            #[doc = r#"wp_tablet_pad_ring.source.finger, a wp_tablet_pad_ring.stop event"#]
            #[doc = r#"will be sent when the user lifts the finger off the device."#]
            #[doc = r#""#]
            #[doc = r#"This event is optional. If the source is unknown for an interaction,"#]
            #[doc = r#"no event is sent."#]
            async fn r#source(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#source: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_ring_v2 -> source");
                let payload = crate::wire::PayloadBuilder::new().put_uint(source).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent whenever the angle on a ring changes."#]
            #[doc = r#""#]
            #[doc = r#"The angle is provided in degrees clockwise from the logical"#]
            #[doc = r#"north of the ring in the pad's current rotation."#]
            async fn r#angle(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#degrees: crate::wire::Fixed,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_ring_v2 -> angle");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_fixed(degrees)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Stop notification for ring events."#]
            #[doc = r#""#]
            #[doc = r#"For some wp_tablet_pad_ring.source types, a wp_tablet_pad_ring.stop"#]
            #[doc = r#"event is sent to notify a client that the interaction with the ring"#]
            #[doc = r#"has terminated. This enables the client to implement kinetic scrolling."#]
            #[doc = r#"See the wp_tablet_pad_ring.source documentation for information on"#]
            #[doc = r#"when this event may be generated."#]
            #[doc = r#""#]
            #[doc = r#"Any wp_tablet_pad_ring.angle events with the same source after this"#]
            #[doc = r#"event should be considered as the start of a new interaction."#]
            async fn r#stop(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_ring_v2 -> stop");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Indicates the end of a set of ring events that logically belong"#]
            #[doc = r#"together. A client is expected to accumulate the data in all events"#]
            #[doc = r#"within the frame before proceeding."#]
            #[doc = r#""#]
            #[doc = r#"All wp_tablet_pad_ring events before a wp_tablet_pad_ring.frame event belong"#]
            #[doc = r#"logically together. For example, on termination of a finger interaction"#]
            #[doc = r#"on a ring the compositor will send a wp_tablet_pad_ring.source event,"#]
            #[doc = r#"a wp_tablet_pad_ring.stop event and a wp_tablet_pad_ring.frame event."#]
            #[doc = r#""#]
            #[doc = r#"A wp_tablet_pad_ring.frame event is sent for every logical event"#]
            #[doc = r#"group, even if the group only contains a single wp_tablet_pad_ring"#]
            #[doc = r#"event. Specifically, a client may get a sequence: angle, frame,"#]
            #[doc = r#"angle, frame, etc."#]
            async fn r#frame(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#time: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_ring_v2 -> frame");
                let payload = crate::wire::PayloadBuilder::new().put_uint(time).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 3, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod zwp_tablet_pad_strip_v2 {
        enum r#Source {}
        #[doc = r#"A linear interaction area, such as the strips found in Wacom Cintiq"#]
        #[doc = r#"models."#]
        #[doc = r#""#]
        #[doc = r#"Events on a strip are logically grouped by the wl_tablet_pad_strip.frame"#]
        #[doc = r#"event."#]
        pub trait r#ZwpTabletPadStripV2 {
            const INTERFACE: &'static str = "zwp_tablet_pad_strip_v2";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("zwp_tablet_pad_strip_v2 -> set_feedback");
                        Self::r#set_feedback(
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    1 => {
                        tracing::debug!("zwp_tablet_pad_strip_v2 -> destroy");
                        Self::r#destroy(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Requests the compositor to use the provided feedback string"#]
            #[doc = r#"associated with this strip. This request should be issued immediately"#]
            #[doc = r#"after a wp_tablet_pad_group.mode_switch event from the corresponding"#]
            #[doc = r#"group is received, or whenever the strip is mapped to a different"#]
            #[doc = r#"action. See wp_tablet_pad_group.mode_switch for more details."#]
            #[doc = r#""#]
            #[doc = r#"Clients are encouraged to provide context-aware descriptions for"#]
            #[doc = r#"the actions associated with the strip, and compositors may use this"#]
            #[doc = r#"information to offer visual feedback about the button layout"#]
            #[doc = r#"(eg. on-screen displays)."#]
            #[doc = r#""#]
            #[doc = r#"The provided string 'description' is a UTF-8 encoded string to be"#]
            #[doc = r#"associated with this ring, and is considered user-visible; general"#]
            #[doc = r#"internationalization rules apply."#]
            #[doc = r#""#]
            #[doc = r#"The serial argument will be that of the last"#]
            #[doc = r#"wp_tablet_pad_group.mode_switch event received for the group of this"#]
            #[doc = r#"strip. Requests providing other serials than the most recent one will be"#]
            #[doc = r#"ignored."#]
            async fn r#set_feedback(
                client: &mut crate::Client,
                r#description: String,
                r#serial: u32,
            ) -> crate::Result<()>;
            #[doc = r#"This destroys the client's resource for this strip object."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Source information for strip events."#]
            #[doc = r#""#]
            #[doc = r#"This event does not occur on its own. It is sent before a"#]
            #[doc = r#"wp_tablet_pad_strip.frame event and carries the source information"#]
            #[doc = r#"for all events within that frame."#]
            #[doc = r#""#]
            #[doc = r#"The source specifies how this event was generated. If the source is"#]
            #[doc = r#"wp_tablet_pad_strip.source.finger, a wp_tablet_pad_strip.stop event"#]
            #[doc = r#"will be sent when the user lifts their finger off the device."#]
            #[doc = r#""#]
            #[doc = r#"This event is optional. If the source is unknown for an interaction,"#]
            #[doc = r#"no event is sent."#]
            async fn r#source(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#source: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_strip_v2 -> source");
                let payload = crate::wire::PayloadBuilder::new().put_uint(source).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent whenever the position on a strip changes."#]
            #[doc = r#""#]
            #[doc = r#"The position is normalized to a range of [0, 65535], the 0-value"#]
            #[doc = r#"represents the top-most and/or left-most position of the strip in"#]
            #[doc = r#"the pad's current rotation."#]
            async fn r#position(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#position: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_strip_v2 -> position");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(position)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Stop notification for strip events."#]
            #[doc = r#""#]
            #[doc = r#"For some wp_tablet_pad_strip.source types, a wp_tablet_pad_strip.stop"#]
            #[doc = r#"event is sent to notify a client that the interaction with the strip"#]
            #[doc = r#"has terminated. This enables the client to implement kinetic"#]
            #[doc = r#"scrolling. See the wp_tablet_pad_strip.source documentation for"#]
            #[doc = r#"information on when this event may be generated."#]
            #[doc = r#""#]
            #[doc = r#"Any wp_tablet_pad_strip.position events with the same source after this"#]
            #[doc = r#"event should be considered as the start of a new interaction."#]
            async fn r#stop(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_strip_v2 -> stop");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Indicates the end of a set of events that represent one logical"#]
            #[doc = r#"hardware strip event. A client is expected to accumulate the data"#]
            #[doc = r#"in all events within the frame before proceeding."#]
            #[doc = r#""#]
            #[doc = r#"All wp_tablet_pad_strip events before a wp_tablet_pad_strip.frame event belong"#]
            #[doc = r#"logically together. For example, on termination of a finger interaction"#]
            #[doc = r#"on a strip the compositor will send a wp_tablet_pad_strip.source event,"#]
            #[doc = r#"a wp_tablet_pad_strip.stop event and a wp_tablet_pad_strip.frame"#]
            #[doc = r#"event."#]
            #[doc = r#""#]
            #[doc = r#"A wp_tablet_pad_strip.frame event is sent for every logical event"#]
            #[doc = r#"group, even if the group only contains a single wp_tablet_pad_strip"#]
            #[doc = r#"event. Specifically, a client may get a sequence: position, frame,"#]
            #[doc = r#"position, frame, etc."#]
            async fn r#frame(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#time: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_strip_v2 -> frame");
                let payload = crate::wire::PayloadBuilder::new().put_uint(time).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 3, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod zwp_tablet_pad_group_v2 {
        #[doc = r#"A pad group describes a distinct (sub)set of buttons, rings and strips"#]
        #[doc = r#"present in the tablet. The criteria of this grouping is usually positional,"#]
        #[doc = r#"eg. if a tablet has buttons on the left and right side, 2 groups will be"#]
        #[doc = r#"presented. The physical arrangement of groups is undisclosed and may"#]
        #[doc = r#"change on the fly."#]
        #[doc = r#""#]
        #[doc = r#"Pad groups will announce their features during pad initialization. Between"#]
        #[doc = r#"the corresponding wp_tablet_pad.group event and wp_tablet_pad_group.done, the"#]
        #[doc = r#"pad group will announce the buttons, rings and strips contained in it,"#]
        #[doc = r#"plus the number of supported modes."#]
        #[doc = r#""#]
        #[doc = r#"Modes are a mechanism to allow multiple groups of actions for every element"#]
        #[doc = r#"in the pad group. The number of groups and available modes in each is"#]
        #[doc = r#"persistent across device plugs. The current mode is user-switchable, it"#]
        #[doc = r#"will be announced through the wp_tablet_pad_group.mode_switch event both"#]
        #[doc = r#"whenever it is switched, and after wp_tablet_pad.enter."#]
        #[doc = r#""#]
        #[doc = r#"The current mode logically applies to all elements in the pad group,"#]
        #[doc = r#"although it is at clients' discretion whether to actually perform different"#]
        #[doc = r#"actions, and/or issue the respective .set_feedback requests to notify the"#]
        #[doc = r#"compositor. See the wp_tablet_pad_group.mode_switch event for more details."#]
        pub trait r#ZwpTabletPadGroupV2 {
            const INTERFACE: &'static str = "zwp_tablet_pad_group_v2";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("zwp_tablet_pad_group_v2 -> destroy");
                        Self::r#destroy(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Destroy the wp_tablet_pad_group object. Objects created from this object"#]
            #[doc = r#"are unaffected and should be destroyed separately."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Sent on wp_tablet_pad_group initialization to announce the available"#]
            #[doc = r#"buttons in the group. Button indices start at 0, a button may only be"#]
            #[doc = r#"in one group at a time."#]
            #[doc = r#""#]
            #[doc = r#"This event is first sent in the initial burst of events before the"#]
            #[doc = r#"wp_tablet_pad_group.done event."#]
            #[doc = r#""#]
            #[doc = r#"Some buttons are reserved by the compositor. These buttons may not be"#]
            #[doc = r#"assigned to any wp_tablet_pad_group. Compositors may broadcast this"#]
            #[doc = r#"event in the case of changes to the mapping of these reserved buttons."#]
            #[doc = r#"If the compositor happens to reserve all buttons in a group, this event"#]
            #[doc = r#"will be sent with an empty array."#]
            async fn r#buttons(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#buttons: Vec<u8>,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_group_v2 -> buttons");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_array(buttons)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent on wp_tablet_pad_group initialization to announce available rings."#]
            #[doc = r#"One event is sent for each ring available on this pad group."#]
            #[doc = r#""#]
            #[doc = r#"This event is sent in the initial burst of events before the"#]
            #[doc = r#"wp_tablet_pad_group.done event."#]
            async fn r#ring(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#ring: crate::wire::ObjectId,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_group_v2 -> ring");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_object(Some(ring))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent on wp_tablet_pad initialization to announce available strips."#]
            #[doc = r#"One event is sent for each strip available on this pad group."#]
            #[doc = r#""#]
            #[doc = r#"This event is sent in the initial burst of events before the"#]
            #[doc = r#"wp_tablet_pad_group.done event."#]
            async fn r#strip(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#strip: crate::wire::ObjectId,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_group_v2 -> strip");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_object(Some(strip))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent on wp_tablet_pad_group initialization to announce that the pad"#]
            #[doc = r#"group may switch between modes. A client may use a mode to store a"#]
            #[doc = r#"specific configuration for buttons, rings and strips and use the"#]
            #[doc = r#"wl_tablet_pad_group.mode_switch event to toggle between these"#]
            #[doc = r#"configurations. Mode indices start at 0."#]
            #[doc = r#""#]
            #[doc = r#"Switching modes is compositor-dependent. See the"#]
            #[doc = r#"wp_tablet_pad_group.mode_switch event for more details."#]
            #[doc = r#""#]
            #[doc = r#"This event is sent in the initial burst of events before the"#]
            #[doc = r#"wp_tablet_pad_group.done event. This event is only sent when more than"#]
            #[doc = r#"more than one mode is available."#]
            async fn r#modes(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#modes: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_group_v2 -> modes");
                let payload = crate::wire::PayloadBuilder::new().put_uint(modes).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 3, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event is sent immediately to signal the end of the initial"#]
            #[doc = r#"burst of descriptive events. A client may consider the static"#]
            #[doc = r#"description of the tablet to be complete and finalize initialization"#]
            #[doc = r#"of the tablet group."#]
            async fn r#done(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_group_v2 -> done");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 4, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Notification that the mode was switched."#]
            #[doc = r#""#]
            #[doc = r#"A mode applies to all buttons, rings and strips in a group"#]
            #[doc = r#"simultaneously, but a client is not required to assign different actions"#]
            #[doc = r#"for each mode. For example, a client may have mode-specific button"#]
            #[doc = r#"mappings but map the ring to vertical scrolling in all modes. Mode"#]
            #[doc = r#"indices start at 0."#]
            #[doc = r#""#]
            #[doc = r#"Switching modes is compositor-dependent. The compositor may provide"#]
            #[doc = r#"visual cues to the client about the mode, e.g. by toggling LEDs on"#]
            #[doc = r#"the tablet device. Mode-switching may be software-controlled or"#]
            #[doc = r#"controlled by one or more physical buttons. For example, on a Wacom"#]
            #[doc = r#"Intuos Pro, the button inside the ring may be assigned to switch"#]
            #[doc = r#"between modes."#]
            #[doc = r#""#]
            #[doc = r#"The compositor will also send this event after wp_tablet_pad.enter on"#]
            #[doc = r#"each group in order to notify of the current mode. Groups that only"#]
            #[doc = r#"feature one mode will use mode=0 when emitting this event."#]
            #[doc = r#""#]
            #[doc = r#"If a button action in the new mode differs from the action in the"#]
            #[doc = r#"previous mode, the client should immediately issue a"#]
            #[doc = r#"wp_tablet_pad.set_feedback request for each changed button."#]
            #[doc = r#""#]
            #[doc = r#"If a ring or strip action in the new mode differs from the action"#]
            #[doc = r#"in the previous mode, the client should immediately issue a"#]
            #[doc = r#"wp_tablet_ring.set_feedback or wp_tablet_strip.set_feedback request"#]
            #[doc = r#"for each changed ring or strip."#]
            async fn r#mode_switch(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#time: u32,
                r#serial: u32,
                r#mode: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_group_v2 -> mode_switch");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_uint(serial)
                    .put_uint(mode)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 5, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod zwp_tablet_pad_v2 {
        enum r#ButtonState {}
        #[doc = r#"A pad device is a set of buttons, rings and strips"#]
        #[doc = r#"usually physically present on the tablet device itself. Some"#]
        #[doc = r#"exceptions exist where the pad device is physically detached, e.g. the"#]
        #[doc = r#"Wacom ExpressKey Remote."#]
        #[doc = r#""#]
        #[doc = r#"Pad devices have no axes that control the cursor and are generally"#]
        #[doc = r#"auxiliary devices to the tool devices used on the tablet surface."#]
        #[doc = r#""#]
        #[doc = r#"A pad device has a number of static characteristics, e.g. the number"#]
        #[doc = r#"of rings. These capabilities are sent in an event sequence after the"#]
        #[doc = r#"wp_tablet_seat.pad_added event before any actual events from this pad."#]
        #[doc = r#"This initial event sequence is terminated by a wp_tablet_pad.done"#]
        #[doc = r#"event."#]
        #[doc = r#""#]
        #[doc = r#"All pad features (buttons, rings and strips) are logically divided into"#]
        #[doc = r#"groups and all pads have at least one group. The available groups are"#]
        #[doc = r#"notified through the wp_tablet_pad.group event; the compositor will"#]
        #[doc = r#"emit one event per group before emitting wp_tablet_pad.done."#]
        #[doc = r#""#]
        #[doc = r#"Groups may have multiple modes. Modes allow clients to map multiple"#]
        #[doc = r#"actions to a single pad feature. Only one mode can be active per group,"#]
        #[doc = r#"although different groups may have different active modes."#]
        pub trait r#ZwpTabletPadV2 {
            const INTERFACE: &'static str = "zwp_tablet_pad_v2";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("zwp_tablet_pad_v2 -> set_feedback");
                        Self::r#set_feedback(
                            client,
                            message.uint()?,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    1 => {
                        tracing::debug!("zwp_tablet_pad_v2 -> destroy");
                        Self::r#destroy(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Requests the compositor to use the provided feedback string"#]
            #[doc = r#"associated with this button. This request should be issued immediately"#]
            #[doc = r#"after a wp_tablet_pad_group.mode_switch event from the corresponding"#]
            #[doc = r#"group is received, or whenever a button is mapped to a different"#]
            #[doc = r#"action. See wp_tablet_pad_group.mode_switch for more details."#]
            #[doc = r#""#]
            #[doc = r#"Clients are encouraged to provide context-aware descriptions for"#]
            #[doc = r#"the actions associated with each button, and compositors may use"#]
            #[doc = r#"this information to offer visual feedback on the button layout"#]
            #[doc = r#"(e.g. on-screen displays)."#]
            #[doc = r#""#]
            #[doc = r#"Button indices start at 0. Setting the feedback string on a button"#]
            #[doc = r#"that is reserved by the compositor (i.e. not belonging to any"#]
            #[doc = r#"wp_tablet_pad_group) does not generate an error but the compositor"#]
            #[doc = r#"is free to ignore the request."#]
            #[doc = r#""#]
            #[doc = r#"The provided string 'description' is a UTF-8 encoded string to be"#]
            #[doc = r#"associated with this ring, and is considered user-visible; general"#]
            #[doc = r#"internationalization rules apply."#]
            #[doc = r#""#]
            #[doc = r#"The serial argument will be that of the last"#]
            #[doc = r#"wp_tablet_pad_group.mode_switch event received for the group of this"#]
            #[doc = r#"button. Requests providing other serials than the most recent one will"#]
            #[doc = r#"be ignored."#]
            async fn r#set_feedback(
                client: &mut crate::Client,
                r#button: u32,
                r#description: String,
                r#serial: u32,
            ) -> crate::Result<()>;
            #[doc = r#"Destroy the wp_tablet_pad object. Objects created from this object"#]
            #[doc = r#"are unaffected and should be destroyed separately."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Sent on wp_tablet_pad initialization to announce available groups."#]
            #[doc = r#"One event is sent for each pad group available."#]
            #[doc = r#""#]
            #[doc = r#"This event is sent in the initial burst of events before the"#]
            #[doc = r#"wp_tablet_pad.done event. At least one group will be announced."#]
            async fn r#group(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#pad_group: crate::wire::ObjectId,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_v2 -> group");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_object(Some(pad_group))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"A system-specific device path that indicates which device is behind"#]
            #[doc = r#"this wp_tablet_pad. This information may be used to gather additional"#]
            #[doc = r#"information about the device, e.g. through libwacom."#]
            #[doc = r#""#]
            #[doc = r#"The format of the path is unspecified, it may be a device node, a"#]
            #[doc = r#"sysfs path, or some other identifier. It is up to the client to"#]
            #[doc = r#"identify the string provided."#]
            #[doc = r#""#]
            #[doc = r#"This event is sent in the initial burst of events before the"#]
            #[doc = r#"wp_tablet_pad.done event."#]
            async fn r#path(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#path: String,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_v2 -> path");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_string(Some(path))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent on wp_tablet_pad initialization to announce the available"#]
            #[doc = r#"buttons."#]
            #[doc = r#""#]
            #[doc = r#"This event is sent in the initial burst of events before the"#]
            #[doc = r#"wp_tablet_pad.done event. This event is only sent when at least one"#]
            #[doc = r#"button is available."#]
            async fn r#buttons(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#buttons: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_v2 -> buttons");
                let payload = crate::wire::PayloadBuilder::new().put_uint(buttons).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event signals the end of the initial burst of descriptive"#]
            #[doc = r#"events. A client may consider the static description of the pad to"#]
            #[doc = r#"be complete and finalize initialization of the pad."#]
            async fn r#done(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_v2 -> done");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 3, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent whenever the physical state of a button changes."#]
            async fn r#button(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#time: u32,
                r#button: u32,
                r#state: u32,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_v2 -> button");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(time)
                    .put_uint(button)
                    .put_uint(state)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 4, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Notification that this pad is focused on the specified surface."#]
            async fn r#enter(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
                r#tablet: crate::wire::ObjectId,
                r#surface: crate::wire::ObjectId,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_v2 -> enter");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(Some(tablet))
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 5, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Notification that this pad is no longer focused on the specified"#]
            #[doc = r#"surface."#]
            async fn r#leave(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
                r#surface: crate::wire::ObjectId,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_v2 -> leave");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_uint(serial)
                    .put_object(Some(surface))
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 6, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"Sent when the pad has been removed from the system. When a tablet"#]
            #[doc = r#"is removed its pad(s) will be removed too."#]
            #[doc = r#""#]
            #[doc = r#"When this event is received, the client must destroy all rings, strips"#]
            #[doc = r#"and groups that were offered by this pad, and issue wp_tablet_pad.destroy"#]
            #[doc = r#"the pad itself."#]
            async fn r#removed(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("zwp_tablet_pad_v2 -> removed");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 7, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
}
pub mod viewporter {
    pub mod wp_viewporter {
        enum r#Error {}
        #[doc = r#"The global interface exposing surface cropping and scaling"#]
        #[doc = r#"capabilities is used to instantiate an interface extension for a"#]
        #[doc = r#"wl_surface object. This extended interface will then allow"#]
        #[doc = r#"cropping and scaling the surface contents, effectively"#]
        #[doc = r#"disconnecting the direct relationship between the buffer and the"#]
        #[doc = r#"surface size."#]
        pub trait r#WpViewporter {
            const INTERFACE: &'static str = "wp_viewporter";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wp_viewporter -> destroy");
                        Self::r#destroy(client).await
                    }
                    1 => {
                        tracing::debug!("wp_viewporter -> get_viewport");
                        Self::r#get_viewport(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Informs the server that the client will not be using this"#]
            #[doc = r#"protocol object anymore. This does not affect any other objects,"#]
            #[doc = r#"wp_viewport objects included."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Instantiate an interface extension for the given wl_surface to"#]
            #[doc = r#"crop and scale its content. If the given wl_surface already has"#]
            #[doc = r#"a wp_viewport object associated, the viewport_exists"#]
            #[doc = r#"protocol error is raised."#]
            async fn r#get_viewport(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
                r#surface: crate::wire::ObjectId,
            ) -> crate::Result<()>;
        }
    }
    pub mod wp_viewport {
        enum r#Error {}
        #[doc = r#"An additional interface to a wl_surface object, which allows the"#]
        #[doc = r#"client to specify the cropping and scaling of the surface"#]
        #[doc = r#"contents."#]
        #[doc = r#""#]
        #[doc = r#"This interface works with two concepts: the source rectangle (src_x,"#]
        #[doc = r#"src_y, src_width, src_height), and the destination size (dst_width,"#]
        #[doc = r#"dst_height). The contents of the source rectangle are scaled to the"#]
        #[doc = r#"destination size, and content outside the source rectangle is ignored."#]
        #[doc = r#"This state is double-buffered, and is applied on the next"#]
        #[doc = r#"wl_surface.commit."#]
        #[doc = r#""#]
        #[doc = r#"The two parts of crop and scale state are independent: the source"#]
        #[doc = r#"rectangle, and the destination size. Initially both are unset, that"#]
        #[doc = r#"is, no scaling is applied. The whole of the current wl_buffer is"#]
        #[doc = r#"used as the source, and the surface size is as defined in"#]
        #[doc = r#"wl_surface.attach."#]
        #[doc = r#""#]
        #[doc = r#"If the destination size is set, it causes the surface size to become"#]
        #[doc = r#"dst_width, dst_height. The source (rectangle) is scaled to exactly"#]
        #[doc = r#"this size. This overrides whatever the attached wl_buffer size is,"#]
        #[doc = r#"unless the wl_buffer is NULL. If the wl_buffer is NULL, the surface"#]
        #[doc = r#"has no content and therefore no size. Otherwise, the size is always"#]
        #[doc = r#"at least 1x1 in surface local coordinates."#]
        #[doc = r#""#]
        #[doc = r#"If the source rectangle is set, it defines what area of the wl_buffer is"#]
        #[doc = r#"taken as the source. If the source rectangle is set and the destination"#]
        #[doc = r#"size is not set, then src_width and src_height must be integers, and the"#]
        #[doc = r#"surface size becomes the source rectangle size. This results in cropping"#]
        #[doc = r#"without scaling. If src_width or src_height are not integers and"#]
        #[doc = r#"destination size is not set, the bad_size protocol error is raised when"#]
        #[doc = r#"the surface state is applied."#]
        #[doc = r#""#]
        #[doc = r#"The coordinate transformations from buffer pixel coordinates up to"#]
        #[doc = r#"the surface-local coordinates happen in the following order:"#]
        #[doc = r#"1. buffer_transform (wl_surface.set_buffer_transform)"#]
        #[doc = r#"2. buffer_scale (wl_surface.set_buffer_scale)"#]
        #[doc = r#"3. crop and scale (wp_viewport.set*)"#]
        #[doc = r#"This means, that the source rectangle coordinates of crop and scale"#]
        #[doc = r#"are given in the coordinates after the buffer transform and scale,"#]
        #[doc = r#"i.e. in the coordinates that would be the surface-local coordinates"#]
        #[doc = r#"if the crop and scale was not applied."#]
        #[doc = r#""#]
        #[doc = r#"If src_x or src_y are negative, the bad_value protocol error is raised."#]
        #[doc = r#"Otherwise, if the source rectangle is partially or completely outside of"#]
        #[doc = r#"the non-NULL wl_buffer, then the out_of_buffer protocol error is raised"#]
        #[doc = r#"when the surface state is applied. A NULL wl_buffer does not raise the"#]
        #[doc = r#"out_of_buffer error."#]
        #[doc = r#""#]
        #[doc = r#"If the wl_surface associated with the wp_viewport is destroyed,"#]
        #[doc = r#"all wp_viewport requests except 'destroy' raise the protocol error"#]
        #[doc = r#"no_surface."#]
        #[doc = r#""#]
        #[doc = r#"If the wp_viewport object is destroyed, the crop and scale"#]
        #[doc = r#"state is removed from the wl_surface. The change will be applied"#]
        #[doc = r#"on the next wl_surface.commit."#]
        pub trait r#WpViewport {
            const INTERFACE: &'static str = "wp_viewport";
            const VERSION: u32 = 1;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("wp_viewport -> destroy");
                        Self::r#destroy(client).await
                    }
                    1 => {
                        tracing::debug!("wp_viewport -> set_source");
                        Self::r#set_source(
                            client,
                            message.fixed()?,
                            message.fixed()?,
                            message.fixed()?,
                            message.fixed()?,
                        )
                        .await
                    }
                    2 => {
                        tracing::debug!("wp_viewport -> set_destination");
                        Self::r#set_destination(client, message.int()?, message.int()?).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"The associated wl_surface's crop and scale state is removed."#]
            #[doc = r#"The change is applied on the next wl_surface.commit."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Set the source rectangle of the associated wl_surface. See"#]
            #[doc = r#"wp_viewport for the description, and relation to the wl_buffer"#]
            #[doc = r#"size."#]
            #[doc = r#""#]
            #[doc = r#"If all of x, y, width and height are -1.0, the source rectangle is"#]
            #[doc = r#"unset instead. Any other set of values where width or height are zero"#]
            #[doc = r#"or negative, or x or y are negative, raise the bad_value protocol"#]
            #[doc = r#"error."#]
            #[doc = r#""#]
            #[doc = r#"The crop and scale state is double-buffered state, and will be"#]
            #[doc = r#"applied on the next wl_surface.commit."#]
            async fn r#set_source(
                client: &mut crate::Client,
                r#x: crate::wire::Fixed,
                r#y: crate::wire::Fixed,
                r#width: crate::wire::Fixed,
                r#height: crate::wire::Fixed,
            ) -> crate::Result<()>;
            #[doc = r#"Set the destination size of the associated wl_surface. See"#]
            #[doc = r#"wp_viewport for the description, and relation to the wl_buffer"#]
            #[doc = r#"size."#]
            #[doc = r#""#]
            #[doc = r#"If width is -1 and height is -1, the destination size is unset"#]
            #[doc = r#"instead. Any other pair of values for width and height that"#]
            #[doc = r#"contains zero or negative values raises the bad_value protocol"#]
            #[doc = r#"error."#]
            #[doc = r#""#]
            #[doc = r#"The crop and scale state is double-buffered state, and will be"#]
            #[doc = r#"applied on the next wl_surface.commit."#]
            async fn r#set_destination(
                client: &mut crate::Client,
                r#width: i32,
                r#height: i32,
            ) -> crate::Result<()>;
        }
    }
}
pub mod xdg_shell {
    pub mod xdg_wm_base {
        enum r#Error {}
        #[doc = r#"The xdg_wm_base interface is exposed as a global object enabling clients"#]
        #[doc = r#"to turn their wl_surfaces into windows in a desktop environment. It"#]
        #[doc = r#"defines the basic functionality needed for clients and the compositor to"#]
        #[doc = r#"create windows that can be dragged, resized, maximized, etc, as well as"#]
        #[doc = r#"creating transient windows such as popup menus."#]
        pub trait r#XdgWmBase {
            const INTERFACE: &'static str = "xdg_wm_base";
            const VERSION: u32 = 6;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("xdg_wm_base -> destroy");
                        Self::r#destroy(client).await
                    }
                    1 => {
                        tracing::debug!("xdg_wm_base -> create_positioner");
                        Self::r#create_positioner(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2 => {
                        tracing::debug!("xdg_wm_base -> get_xdg_surface");
                        Self::r#get_xdg_surface(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    3 => {
                        tracing::debug!("xdg_wm_base -> pong");
                        Self::r#pong(client, message.uint()?).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Destroy this xdg_wm_base object."#]
            #[doc = r#""#]
            #[doc = r#"Destroying a bound xdg_wm_base object while there are surfaces"#]
            #[doc = r#"still alive created by this xdg_wm_base object instance is illegal"#]
            #[doc = r#"and will result in a defunct_surfaces error."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Create a positioner object. A positioner object is used to position"#]
            #[doc = r#"surfaces relative to some parent surface. See the interface description"#]
            #[doc = r#"and xdg_surface.get_popup for details."#]
            async fn r#create_positioner(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"This creates an xdg_surface for the given surface. While xdg_surface"#]
            #[doc = r#"itself is not a role, the corresponding surface may only be assigned"#]
            #[doc = r#"a role extending xdg_surface, such as xdg_toplevel or xdg_popup. It is"#]
            #[doc = r#"illegal to create an xdg_surface for a wl_surface which already has an"#]
            #[doc = r#"assigned role and this will result in a role error."#]
            #[doc = r#""#]
            #[doc = r#"This creates an xdg_surface for the given surface. An xdg_surface is"#]
            #[doc = r#"used as basis to define a role to a given surface, such as xdg_toplevel"#]
            #[doc = r#"or xdg_popup. It also manages functionality shared between xdg_surface"#]
            #[doc = r#"based surface roles."#]
            #[doc = r#""#]
            #[doc = r#"See the documentation of xdg_surface for more details about what an"#]
            #[doc = r#"xdg_surface is and how it is used."#]
            async fn r#get_xdg_surface(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
                r#surface: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"A client must respond to a ping event with a pong request or"#]
            #[doc = r#"the client may be deemed unresponsive. See xdg_wm_base.ping"#]
            #[doc = r#"and xdg_wm_base.error.unresponsive."#]
            async fn r#pong(client: &mut crate::Client, r#serial: u32) -> crate::Result<()>;
            #[doc = r#"The ping event asks the client if it's still alive. Pass the"#]
            #[doc = r#"serial specified in the event back to the compositor by sending"#]
            #[doc = r#"a "pong" request back with the specified serial. See xdg_wm_base.pong."#]
            #[doc = r#""#]
            #[doc = r#"Compositors can use this to determine if the client is still"#]
            #[doc = r#"alive. It's unspecified what will happen if the client doesn't"#]
            #[doc = r#"respond to the ping request, or in what timeframe. Clients should"#]
            #[doc = r#"try to respond in a reasonable amount of time. The “unresponsive”"#]
            #[doc = r#"error is provided for compositors that wish to disconnect unresponsive"#]
            #[doc = r#"clients."#]
            #[doc = r#""#]
            #[doc = r#"A compositor is free to ping in any way it wants, but a client must"#]
            #[doc = r#"always respond to any xdg_wm_base object it created."#]
            async fn r#ping(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
            ) -> crate::Result<()> {
                tracing::debug!("xdg_wm_base -> ping");
                let payload = crate::wire::PayloadBuilder::new().put_uint(serial).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod xdg_positioner {
        enum r#Error {}
        enum r#Anchor {}
        enum r#Gravity {}
        enum r#ConstraintAdjustment {}
        #[doc = r#"The xdg_positioner provides a collection of rules for the placement of a"#]
        #[doc = r#"child surface relative to a parent surface. Rules can be defined to ensure"#]
        #[doc = r#"the child surface remains within the visible area's borders, and to"#]
        #[doc = r#"specify how the child surface changes its position, such as sliding along"#]
        #[doc = r#"an axis, or flipping around a rectangle. These positioner-created rules are"#]
        #[doc = r#"constrained by the requirement that a child surface must intersect with or"#]
        #[doc = r#"be at least partially adjacent to its parent surface."#]
        #[doc = r#""#]
        #[doc = r#"See the various requests for details about possible rules."#]
        #[doc = r#""#]
        #[doc = r#"At the time of the request, the compositor makes a copy of the rules"#]
        #[doc = r#"specified by the xdg_positioner. Thus, after the request is complete the"#]
        #[doc = r#"xdg_positioner object can be destroyed or reused; further changes to the"#]
        #[doc = r#"object will have no effect on previous usages."#]
        #[doc = r#""#]
        #[doc = r#"For an xdg_positioner object to be considered complete, it must have a"#]
        #[doc = r#"non-zero size set by set_size, and a non-zero anchor rectangle set by"#]
        #[doc = r#"set_anchor_rect. Passing an incomplete xdg_positioner object when"#]
        #[doc = r#"positioning a surface raises an invalid_positioner error."#]
        pub trait r#XdgPositioner {
            const INTERFACE: &'static str = "xdg_positioner";
            const VERSION: u32 = 6;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("xdg_positioner -> destroy");
                        Self::r#destroy(client).await
                    }
                    1 => {
                        tracing::debug!("xdg_positioner -> set_size");
                        Self::r#set_size(client, message.int()?, message.int()?).await
                    }
                    2 => {
                        tracing::debug!("xdg_positioner -> set_anchor_rect");
                        Self::r#set_anchor_rect(
                            client,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    3 => {
                        tracing::debug!("xdg_positioner -> set_anchor");
                        Self::r#set_anchor(client, message.uint()?).await
                    }
                    4 => {
                        tracing::debug!("xdg_positioner -> set_gravity");
                        Self::r#set_gravity(client, message.uint()?).await
                    }
                    5 => {
                        tracing::debug!("xdg_positioner -> set_constraint_adjustment");
                        Self::r#set_constraint_adjustment(client, message.uint()?).await
                    }
                    6 => {
                        tracing::debug!("xdg_positioner -> set_offset");
                        Self::r#set_offset(client, message.int()?, message.int()?).await
                    }
                    7 => {
                        tracing::debug!("xdg_positioner -> set_reactive");
                        Self::r#set_reactive(client).await
                    }
                    8 => {
                        tracing::debug!("xdg_positioner -> set_parent_size");
                        Self::r#set_parent_size(client, message.int()?, message.int()?).await
                    }
                    9 => {
                        tracing::debug!("xdg_positioner -> set_parent_configure");
                        Self::r#set_parent_configure(client, message.uint()?).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Notify the compositor that the xdg_positioner will no longer be used."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Set the size of the surface that is to be positioned with the positioner"#]
            #[doc = r#"object. The size is in surface-local coordinates and corresponds to the"#]
            #[doc = r#"window geometry. See xdg_surface.set_window_geometry."#]
            #[doc = r#""#]
            #[doc = r#"If a zero or negative size is set the invalid_input error is raised."#]
            async fn r#set_size(
                client: &mut crate::Client,
                r#width: i32,
                r#height: i32,
            ) -> crate::Result<()>;
            #[doc = r#"Specify the anchor rectangle within the parent surface that the child"#]
            #[doc = r#"surface will be placed relative to. The rectangle is relative to the"#]
            #[doc = r#"window geometry as defined by xdg_surface.set_window_geometry of the"#]
            #[doc = r#"parent surface."#]
            #[doc = r#""#]
            #[doc = r#"When the xdg_positioner object is used to position a child surface, the"#]
            #[doc = r#"anchor rectangle may not extend outside the window geometry of the"#]
            #[doc = r#"positioned child's parent surface."#]
            #[doc = r#""#]
            #[doc = r#"If a negative size is set the invalid_input error is raised."#]
            async fn r#set_anchor_rect(
                client: &mut crate::Client,
                r#x: i32,
                r#y: i32,
                r#width: i32,
                r#height: i32,
            ) -> crate::Result<()>;
            #[doc = r#"Defines the anchor point for the anchor rectangle. The specified anchor"#]
            #[doc = r#"is used derive an anchor point that the child surface will be"#]
            #[doc = r#"positioned relative to. If a corner anchor is set (e.g. 'top_left' or"#]
            #[doc = r#"'bottom_right'), the anchor point will be at the specified corner;"#]
            #[doc = r#"otherwise, the derived anchor point will be centered on the specified"#]
            #[doc = r#"edge, or in the center of the anchor rectangle if no edge is specified."#]
            async fn r#set_anchor(client: &mut crate::Client, r#anchor: u32) -> crate::Result<()>;
            #[doc = r#"Defines in what direction a surface should be positioned, relative to"#]
            #[doc = r#"the anchor point of the parent surface. If a corner gravity is"#]
            #[doc = r#"specified (e.g. 'bottom_right' or 'top_left'), then the child surface"#]
            #[doc = r#"will be placed towards the specified gravity; otherwise, the child"#]
            #[doc = r#"surface will be centered over the anchor point on any axis that had no"#]
            #[doc = r#"gravity specified. If the gravity is not in the ‘gravity’ enum, an"#]
            #[doc = r#"invalid_input error is raised."#]
            async fn r#set_gravity(client: &mut crate::Client, r#gravity: u32)
                -> crate::Result<()>;
            #[doc = r#"Specify how the window should be positioned if the originally intended"#]
            #[doc = r#"position caused the surface to be constrained, meaning at least"#]
            #[doc = r#"partially outside positioning boundaries set by the compositor. The"#]
            #[doc = r#"adjustment is set by constructing a bitmask describing the adjustment to"#]
            #[doc = r#"be made when the surface is constrained on that axis."#]
            #[doc = r#""#]
            #[doc = r#"If no bit for one axis is set, the compositor will assume that the child"#]
            #[doc = r#"surface should not change its position on that axis when constrained."#]
            #[doc = r#""#]
            #[doc = r#"If more than one bit for one axis is set, the order of how adjustments"#]
            #[doc = r#"are applied is specified in the corresponding adjustment descriptions."#]
            #[doc = r#""#]
            #[doc = r#"The default adjustment is none."#]
            async fn r#set_constraint_adjustment(
                client: &mut crate::Client,
                r#constraint_adjustment: u32,
            ) -> crate::Result<()>;
            #[doc = r#"Specify the surface position offset relative to the position of the"#]
            #[doc = r#"anchor on the anchor rectangle and the anchor on the surface. For"#]
            #[doc = r#"example if the anchor of the anchor rectangle is at (x, y), the surface"#]
            #[doc = r#"has the gravity bottom|right, and the offset is (ox, oy), the calculated"#]
            #[doc = r#"surface position will be (x + ox, y + oy). The offset position of the"#]
            #[doc = r#"surface is the one used for constraint testing. See"#]
            #[doc = r#"set_constraint_adjustment."#]
            #[doc = r#""#]
            #[doc = r#"An example use case is placing a popup menu on top of a user interface"#]
            #[doc = r#"element, while aligning the user interface element of the parent surface"#]
            #[doc = r#"with some user interface element placed somewhere in the popup surface."#]
            async fn r#set_offset(
                client: &mut crate::Client,
                r#x: i32,
                r#y: i32,
            ) -> crate::Result<()>;
            #[doc = r#"When set reactive, the surface is reconstrained if the conditions used"#]
            #[doc = r#"for constraining changed, e.g. the parent window moved."#]
            #[doc = r#""#]
            #[doc = r#"If the conditions changed and the popup was reconstrained, an"#]
            #[doc = r#"xdg_popup.configure event is sent with updated geometry, followed by an"#]
            #[doc = r#"xdg_surface.configure event."#]
            async fn r#set_reactive(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Set the parent window geometry the compositor should use when"#]
            #[doc = r#"positioning the popup. The compositor may use this information to"#]
            #[doc = r#"determine the future state the popup should be constrained using. If"#]
            #[doc = r#"this doesn't match the dimension of the parent the popup is eventually"#]
            #[doc = r#"positioned against, the behavior is undefined."#]
            #[doc = r#""#]
            #[doc = r#"The arguments are given in the surface-local coordinate space."#]
            async fn r#set_parent_size(
                client: &mut crate::Client,
                r#parent_width: i32,
                r#parent_height: i32,
            ) -> crate::Result<()>;
            #[doc = r#"Set the serial of an xdg_surface.configure event this positioner will be"#]
            #[doc = r#"used in response to. The compositor may use this information together"#]
            #[doc = r#"with set_parent_size to determine what future state the popup should be"#]
            #[doc = r#"constrained using."#]
            async fn r#set_parent_configure(
                client: &mut crate::Client,
                r#serial: u32,
            ) -> crate::Result<()>;
        }
    }
    pub mod xdg_surface {
        enum r#Error {}
        #[doc = r#"An interface that may be implemented by a wl_surface, for"#]
        #[doc = r#"implementations that provide a desktop-style user interface."#]
        #[doc = r#""#]
        #[doc = r#"It provides a base set of functionality required to construct user"#]
        #[doc = r#"interface elements requiring management by the compositor, such as"#]
        #[doc = r#"toplevel windows, menus, etc. The types of functionality are split into"#]
        #[doc = r#"xdg_surface roles."#]
        #[doc = r#""#]
        #[doc = r#"Creating an xdg_surface does not set the role for a wl_surface. In order"#]
        #[doc = r#"to map an xdg_surface, the client must create a role-specific object"#]
        #[doc = r#"using, e.g., get_toplevel, get_popup. The wl_surface for any given"#]
        #[doc = r#"xdg_surface can have at most one role, and may not be assigned any role"#]
        #[doc = r#"not based on xdg_surface."#]
        #[doc = r#""#]
        #[doc = r#"A role must be assigned before any other requests are made to the"#]
        #[doc = r#"xdg_surface object."#]
        #[doc = r#""#]
        #[doc = r#"The client must call wl_surface.commit on the corresponding wl_surface"#]
        #[doc = r#"for the xdg_surface state to take effect."#]
        #[doc = r#""#]
        #[doc = r#"Creating an xdg_surface from a wl_surface which has a buffer attached or"#]
        #[doc = r#"committed is a client error, and any attempts by a client to attach or"#]
        #[doc = r#"manipulate a buffer prior to the first xdg_surface.configure call must"#]
        #[doc = r#"also be treated as errors."#]
        #[doc = r#""#]
        #[doc = r#"After creating a role-specific object and setting it up, the client must"#]
        #[doc = r#"perform an initial commit without any buffer attached. The compositor"#]
        #[doc = r#"will reply with initial wl_surface state such as"#]
        #[doc = r#"wl_surface.preferred_buffer_scale followed by an xdg_surface.configure"#]
        #[doc = r#"event. The client must acknowledge it and is then allowed to attach a"#]
        #[doc = r#"buffer to map the surface."#]
        #[doc = r#""#]
        #[doc = r#"Mapping an xdg_surface-based role surface is defined as making it"#]
        #[doc = r#"possible for the surface to be shown by the compositor. Note that"#]
        #[doc = r#"a mapped surface is not guaranteed to be visible once it is mapped."#]
        #[doc = r#""#]
        #[doc = r#"For an xdg_surface to be mapped by the compositor, the following"#]
        #[doc = r#"conditions must be met:"#]
        #[doc = r#"(1) the client has assigned an xdg_surface-based role to the surface"#]
        #[doc = r#"(2) the client has set and committed the xdg_surface state and the"#]
        #[doc = r#"role-dependent state to the surface"#]
        #[doc = r#"(3) the client has committed a buffer to the surface"#]
        #[doc = r#""#]
        #[doc = r#"A newly-unmapped surface is considered to have met condition (1) out"#]
        #[doc = r#"of the 3 required conditions for mapping a surface if its role surface"#]
        #[doc = r#"has not been destroyed, i.e. the client must perform the initial commit"#]
        #[doc = r#"again before attaching a buffer."#]
        pub trait r#XdgSurface {
            const INTERFACE: &'static str = "xdg_surface";
            const VERSION: u32 = 6;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("xdg_surface -> destroy");
                        Self::r#destroy(client).await
                    }
                    1 => {
                        tracing::debug!("xdg_surface -> get_toplevel");
                        Self::r#get_toplevel(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    2 => {
                        tracing::debug!("xdg_surface -> get_popup");
                        Self::r#get_popup(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.object()?,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    3 => {
                        tracing::debug!("xdg_surface -> set_window_geometry");
                        Self::r#set_window_geometry(
                            client,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    4 => {
                        tracing::debug!("xdg_surface -> ack_configure");
                        Self::r#ack_configure(client, message.uint()?).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"Destroy the xdg_surface object. An xdg_surface must only be destroyed"#]
            #[doc = r#"after its role object has been destroyed, otherwise"#]
            #[doc = r#"a defunct_role_object error is raised."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"This creates an xdg_toplevel object for the given xdg_surface and gives"#]
            #[doc = r#"the associated wl_surface the xdg_toplevel role."#]
            #[doc = r#""#]
            #[doc = r#"See the documentation of xdg_toplevel for more details about what an"#]
            #[doc = r#"xdg_toplevel is and how it is used."#]
            async fn r#get_toplevel(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"This creates an xdg_popup object for the given xdg_surface and gives"#]
            #[doc = r#"the associated wl_surface the xdg_popup role."#]
            #[doc = r#""#]
            #[doc = r#"If null is passed as a parent, a parent surface must be specified using"#]
            #[doc = r#"some other protocol, before committing the initial state."#]
            #[doc = r#""#]
            #[doc = r#"See the documentation of xdg_popup for more details about what an"#]
            #[doc = r#"xdg_popup is and how it is used."#]
            async fn r#get_popup(
                client: &mut crate::Client,
                r#id: crate::wire::ObjectId,
                r#parent: Option<crate::wire::ObjectId>,
                r#positioner: crate::wire::ObjectId,
            ) -> crate::Result<()>;
            #[doc = r#"The window geometry of a surface is its "visible bounds" from the"#]
            #[doc = r#"user's perspective. Client-side decorations often have invisible"#]
            #[doc = r#"portions like drop-shadows which should be ignored for the"#]
            #[doc = r#"purposes of aligning, placing and constraining windows."#]
            #[doc = r#""#]
            #[doc = r#"The window geometry is double buffered, and will be applied at the"#]
            #[doc = r#"time wl_surface.commit of the corresponding wl_surface is called."#]
            #[doc = r#""#]
            #[doc = r#"When maintaining a position, the compositor should treat the (x, y)"#]
            #[doc = r#"coordinate of the window geometry as the top left corner of the window."#]
            #[doc = r#"A client changing the (x, y) window geometry coordinate should in"#]
            #[doc = r#"general not alter the position of the window."#]
            #[doc = r#""#]
            #[doc = r#"Once the window geometry of the surface is set, it is not possible to"#]
            #[doc = r#"unset it, and it will remain the same until set_window_geometry is"#]
            #[doc = r#"called again, even if a new subsurface or buffer is attached."#]
            #[doc = r#""#]
            #[doc = r#"If never set, the value is the full bounds of the surface,"#]
            #[doc = r#"including any subsurfaces. This updates dynamically on every"#]
            #[doc = r#"commit. This unset is meant for extremely simple clients."#]
            #[doc = r#""#]
            #[doc = r#"The arguments are given in the surface-local coordinate space of"#]
            #[doc = r#"the wl_surface associated with this xdg_surface, and may extend outside"#]
            #[doc = r#"of the wl_surface itself to mark parts of the subsurface tree as part of"#]
            #[doc = r#"the window geometry."#]
            #[doc = r#""#]
            #[doc = r#"When applied, the effective window geometry will be the set window"#]
            #[doc = r#"geometry clamped to the bounding rectangle of the combined"#]
            #[doc = r#"geometry of the surface of the xdg_surface and the associated"#]
            #[doc = r#"subsurfaces."#]
            #[doc = r#""#]
            #[doc = r#"The effective geometry will not be recalculated unless a new call to"#]
            #[doc = r#"set_window_geometry is done and the new pending surface state is"#]
            #[doc = r#"subsequently applied."#]
            #[doc = r#""#]
            #[doc = r#"The width and height of the effective window geometry must be"#]
            #[doc = r#"greater than zero. Setting an invalid size will raise an"#]
            #[doc = r#"invalid_size error."#]
            async fn r#set_window_geometry(
                client: &mut crate::Client,
                r#x: i32,
                r#y: i32,
                r#width: i32,
                r#height: i32,
            ) -> crate::Result<()>;
            #[doc = r#"When a configure event is received, if a client commits the"#]
            #[doc = r#"surface in response to the configure event, then the client"#]
            #[doc = r#"must make an ack_configure request sometime before the commit"#]
            #[doc = r#"request, passing along the serial of the configure event."#]
            #[doc = r#""#]
            #[doc = r#"For instance, for toplevel surfaces the compositor might use this"#]
            #[doc = r#"information to move a surface to the top left only when the client has"#]
            #[doc = r#"drawn itself for the maximized or fullscreen state."#]
            #[doc = r#""#]
            #[doc = r#"If the client receives multiple configure events before it"#]
            #[doc = r#"can respond to one, it only has to ack the last configure event."#]
            #[doc = r#"Acking a configure event that was never sent raises an invalid_serial"#]
            #[doc = r#"error."#]
            #[doc = r#""#]
            #[doc = r#"A client is not required to commit immediately after sending"#]
            #[doc = r#"an ack_configure request - it may even ack_configure several times"#]
            #[doc = r#"before its next surface commit."#]
            #[doc = r#""#]
            #[doc = r#"A client may send multiple ack_configure requests before committing, but"#]
            #[doc = r#"only the last request sent before a commit indicates which configure"#]
            #[doc = r#"event the client really is responding to."#]
            #[doc = r#""#]
            #[doc = r#"Sending an ack_configure request consumes the serial number sent with"#]
            #[doc = r#"the request, as well as serial numbers sent by all configure events"#]
            #[doc = r#"sent on this xdg_surface prior to the configure event referenced by"#]
            #[doc = r#"the committed serial."#]
            #[doc = r#""#]
            #[doc = r#"It is an error to issue multiple ack_configure requests referencing a"#]
            #[doc = r#"serial from the same configure event, or to issue an ack_configure"#]
            #[doc = r#"request referencing a serial from a configure event issued before the"#]
            #[doc = r#"event identified by the last ack_configure request for the same"#]
            #[doc = r#"xdg_surface. Doing so will raise an invalid_serial error."#]
            async fn r#ack_configure(
                client: &mut crate::Client,
                r#serial: u32,
            ) -> crate::Result<()>;
            #[doc = r#"The configure event marks the end of a configure sequence. A configure"#]
            #[doc = r#"sequence is a set of one or more events configuring the state of the"#]
            #[doc = r#"xdg_surface, including the final xdg_surface.configure event."#]
            #[doc = r#""#]
            #[doc = r#"Where applicable, xdg_surface surface roles will during a configure"#]
            #[doc = r#"sequence extend this event as a latched state sent as events before the"#]
            #[doc = r#"xdg_surface.configure event. Such events should be considered to make up"#]
            #[doc = r#"a set of atomically applied configuration states, where the"#]
            #[doc = r#"xdg_surface.configure commits the accumulated state."#]
            #[doc = r#""#]
            #[doc = r#"Clients should arrange their surface for the new states, and then send"#]
            #[doc = r#"an ack_configure request with the serial sent in this configure event at"#]
            #[doc = r#"some point before committing the new surface."#]
            #[doc = r#""#]
            #[doc = r#"If the client receives multiple configure events before it can respond"#]
            #[doc = r#"to one, it is free to discard all but the last event it received."#]
            async fn r#configure(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#serial: u32,
            ) -> crate::Result<()> {
                tracing::debug!("xdg_surface -> configure");
                let payload = crate::wire::PayloadBuilder::new().put_uint(serial).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod xdg_toplevel {
        enum r#Error {}
        enum r#ResizeEdge {}
        enum r#State {}
        enum r#WmCapabilities {}
        #[doc = r#"This interface defines an xdg_surface role which allows a surface to,"#]
        #[doc = r#"among other things, set window-like properties such as maximize,"#]
        #[doc = r#"fullscreen, and minimize, set application-specific metadata like title and"#]
        #[doc = r#"id, and well as trigger user interactive operations such as interactive"#]
        #[doc = r#"resize and move."#]
        #[doc = r#""#]
        #[doc = r#"A xdg_toplevel by default is responsible for providing the full intended"#]
        #[doc = r#"visual representation of the toplevel, which depending on the window"#]
        #[doc = r#"state, may mean things like a title bar, window controls and drop shadow."#]
        #[doc = r#""#]
        #[doc = r#"Unmapping an xdg_toplevel means that the surface cannot be shown"#]
        #[doc = r#"by the compositor until it is explicitly mapped again."#]
        #[doc = r#"All active operations (e.g., move, resize) are canceled and all"#]
        #[doc = r#"attributes (e.g. title, state, stacking, ...) are discarded for"#]
        #[doc = r#"an xdg_toplevel surface when it is unmapped. The xdg_toplevel returns to"#]
        #[doc = r#"the state it had right after xdg_surface.get_toplevel. The client"#]
        #[doc = r#"can re-map the toplevel by perfoming a commit without any buffer"#]
        #[doc = r#"attached, waiting for a configure event and handling it as usual (see"#]
        #[doc = r#"xdg_surface description)."#]
        #[doc = r#""#]
        #[doc = r#"Attaching a null buffer to a toplevel unmaps the surface."#]
        pub trait r#XdgToplevel {
            const INTERFACE: &'static str = "xdg_toplevel";
            const VERSION: u32 = 6;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("xdg_toplevel -> destroy");
                        Self::r#destroy(client).await
                    }
                    1 => {
                        tracing::debug!("xdg_toplevel -> set_parent");
                        Self::r#set_parent(client, message.object()?).await
                    }
                    2 => {
                        tracing::debug!("xdg_toplevel -> set_title");
                        Self::r#set_title(
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    3 => {
                        tracing::debug!("xdg_toplevel -> set_app_id");
                        Self::r#set_app_id(
                            client,
                            message
                                .string()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                        )
                        .await
                    }
                    4 => {
                        tracing::debug!("xdg_toplevel -> show_window_menu");
                        Self::r#show_window_menu(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                            message.int()?,
                            message.int()?,
                        )
                        .await
                    }
                    5 => {
                        tracing::debug!("xdg_toplevel -> move");
                        Self::r#move(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    6 => {
                        tracing::debug!("xdg_toplevel -> resize");
                        Self::r#resize(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                            message.uint()?,
                        )
                        .await
                    }
                    7 => {
                        tracing::debug!("xdg_toplevel -> set_max_size");
                        Self::r#set_max_size(client, message.int()?, message.int()?).await
                    }
                    8 => {
                        tracing::debug!("xdg_toplevel -> set_min_size");
                        Self::r#set_min_size(client, message.int()?, message.int()?).await
                    }
                    9 => {
                        tracing::debug!("xdg_toplevel -> set_maximized");
                        Self::r#set_maximized(client).await
                    }
                    10 => {
                        tracing::debug!("xdg_toplevel -> unset_maximized");
                        Self::r#unset_maximized(client).await
                    }
                    11 => {
                        tracing::debug!("xdg_toplevel -> set_fullscreen");
                        Self::r#set_fullscreen(client, message.object()?).await
                    }
                    12 => {
                        tracing::debug!("xdg_toplevel -> unset_fullscreen");
                        Self::r#unset_fullscreen(client).await
                    }
                    13 => {
                        tracing::debug!("xdg_toplevel -> set_minimized");
                        Self::r#set_minimized(client).await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"This request destroys the role surface and unmaps the surface;"#]
            #[doc = r#"see "Unmapping" behavior in interface section for details."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Set the "parent" of this surface. This surface should be stacked"#]
            #[doc = r#"above the parent surface and all other ancestor surfaces."#]
            #[doc = r#""#]
            #[doc = r#"Parent surfaces should be set on dialogs, toolboxes, or other"#]
            #[doc = r#""auxiliary" surfaces, so that the parent is raised when the dialog"#]
            #[doc = r#"is raised."#]
            #[doc = r#""#]
            #[doc = r#"Setting a null parent for a child surface unsets its parent. Setting"#]
            #[doc = r#"a null parent for a surface which currently has no parent is a no-op."#]
            #[doc = r#""#]
            #[doc = r#"Only mapped surfaces can have child surfaces. Setting a parent which"#]
            #[doc = r#"is not mapped is equivalent to setting a null parent. If a surface"#]
            #[doc = r#"becomes unmapped, its children's parent is set to the parent of"#]
            #[doc = r#"the now-unmapped surface. If the now-unmapped surface has no parent,"#]
            #[doc = r#"its children's parent is unset. If the now-unmapped surface becomes"#]
            #[doc = r#"mapped again, its parent-child relationship is not restored."#]
            #[doc = r#""#]
            #[doc = r#"The parent toplevel must not be one of the child toplevel's"#]
            #[doc = r#"descendants, and the parent must be different from the child toplevel,"#]
            #[doc = r#"otherwise the invalid_parent protocol error is raised."#]
            async fn r#set_parent(
                client: &mut crate::Client,
                r#parent: Option<crate::wire::ObjectId>,
            ) -> crate::Result<()>;
            #[doc = r#"Set a short title for the surface."#]
            #[doc = r#""#]
            #[doc = r#"This string may be used to identify the surface in a task bar,"#]
            #[doc = r#"window list, or other user interface elements provided by the"#]
            #[doc = r#"compositor."#]
            #[doc = r#""#]
            #[doc = r#"The string must be encoded in UTF-8."#]
            async fn r#set_title(client: &mut crate::Client, r#title: String) -> crate::Result<()>;
            #[doc = r#"Set an application identifier for the surface."#]
            #[doc = r#""#]
            #[doc = r#"The app ID identifies the general class of applications to which"#]
            #[doc = r#"the surface belongs. The compositor can use this to group multiple"#]
            #[doc = r#"surfaces together, or to determine how to launch a new application."#]
            #[doc = r#""#]
            #[doc = r#"For D-Bus activatable applications, the app ID is used as the D-Bus"#]
            #[doc = r#"service name."#]
            #[doc = r#""#]
            #[doc = r#"The compositor shell will try to group application surfaces together"#]
            #[doc = r#"by their app ID. As a best practice, it is suggested to select app"#]
            #[doc = r#"ID's that match the basename of the application's .desktop file."#]
            #[doc = r#"For example, "org.freedesktop.FooViewer" where the .desktop file is"#]
            #[doc = r#""org.freedesktop.FooViewer.desktop"."#]
            #[doc = r#""#]
            #[doc = r#"Like other properties, a set_app_id request can be sent after the"#]
            #[doc = r#"xdg_toplevel has been mapped to update the property."#]
            #[doc = r#""#]
            #[doc = r#"See the desktop-entry specification [0] for more details on"#]
            #[doc = r#"application identifiers and how they relate to well-known D-Bus"#]
            #[doc = r#"names and .desktop files."#]
            #[doc = r#""#]
            #[doc = r#"[0] https://standards.freedesktop.org/desktop-entry-spec/"#]
            async fn r#set_app_id(
                client: &mut crate::Client,
                r#app_id: String,
            ) -> crate::Result<()>;
            #[doc = r#"Clients implementing client-side decorations might want to show"#]
            #[doc = r#"a context menu when right-clicking on the decorations, giving the"#]
            #[doc = r#"user a menu that they can use to maximize or minimize the window."#]
            #[doc = r#""#]
            #[doc = r#"This request asks the compositor to pop up such a window menu at"#]
            #[doc = r#"the given position, relative to the local surface coordinates of"#]
            #[doc = r#"the parent surface. There are no guarantees as to what menu items"#]
            #[doc = r#"the window menu contains, or even if a window menu will be drawn"#]
            #[doc = r#"at all."#]
            #[doc = r#""#]
            #[doc = r#"This request must be used in response to some sort of user action"#]
            #[doc = r#"like a button press, key press, or touch down event."#]
            async fn r#show_window_menu(
                client: &mut crate::Client,
                r#seat: crate::wire::ObjectId,
                r#serial: u32,
                r#x: i32,
                r#y: i32,
            ) -> crate::Result<()>;
            #[doc = r#"Start an interactive, user-driven move of the surface."#]
            #[doc = r#""#]
            #[doc = r#"This request must be used in response to some sort of user action"#]
            #[doc = r#"like a button press, key press, or touch down event. The passed"#]
            #[doc = r#"serial is used to determine the type of interactive move (touch,"#]
            #[doc = r#"pointer, etc)."#]
            #[doc = r#""#]
            #[doc = r#"The server may ignore move requests depending on the state of"#]
            #[doc = r#"the surface (e.g. fullscreen or maximized), or if the passed serial"#]
            #[doc = r#"is no longer valid."#]
            #[doc = r#""#]
            #[doc = r#"If triggered, the surface will lose the focus of the device"#]
            #[doc = r#"(wl_pointer, wl_touch, etc) used for the move. It is up to the"#]
            #[doc = r#"compositor to visually indicate that the move is taking place, such as"#]
            #[doc = r#"updating a pointer cursor, during the move. There is no guarantee"#]
            #[doc = r#"that the device focus will return when the move is completed."#]
            async fn r#move(
                client: &mut crate::Client,
                r#seat: crate::wire::ObjectId,
                r#serial: u32,
            ) -> crate::Result<()>;
            #[doc = r#"Start a user-driven, interactive resize of the surface."#]
            #[doc = r#""#]
            #[doc = r#"This request must be used in response to some sort of user action"#]
            #[doc = r#"like a button press, key press, or touch down event. The passed"#]
            #[doc = r#"serial is used to determine the type of interactive resize (touch,"#]
            #[doc = r#"pointer, etc)."#]
            #[doc = r#""#]
            #[doc = r#"The server may ignore resize requests depending on the state of"#]
            #[doc = r#"the surface (e.g. fullscreen or maximized)."#]
            #[doc = r#""#]
            #[doc = r#"If triggered, the client will receive configure events with the"#]
            #[doc = r#""resize" state enum value and the expected sizes. See the "resize""#]
            #[doc = r#"enum value for more details about what is required. The client"#]
            #[doc = r#"must also acknowledge configure events using "ack_configure". After"#]
            #[doc = r#"the resize is completed, the client will receive another "configure""#]
            #[doc = r#"event without the resize state."#]
            #[doc = r#""#]
            #[doc = r#"If triggered, the surface also will lose the focus of the device"#]
            #[doc = r#"(wl_pointer, wl_touch, etc) used for the resize. It is up to the"#]
            #[doc = r#"compositor to visually indicate that the resize is taking place,"#]
            #[doc = r#"such as updating a pointer cursor, during the resize. There is no"#]
            #[doc = r#"guarantee that the device focus will return when the resize is"#]
            #[doc = r#"completed."#]
            #[doc = r#""#]
            #[doc = r#"The edges parameter specifies how the surface should be resized, and"#]
            #[doc = r#"is one of the values of the resize_edge enum. Values not matching"#]
            #[doc = r#"a variant of the enum will cause the invalid_resize_edge protocol error."#]
            #[doc = r#"The compositor may use this information to update the surface position"#]
            #[doc = r#"for example when dragging the top left corner. The compositor may also"#]
            #[doc = r#"use this information to adapt its behavior, e.g. choose an appropriate"#]
            #[doc = r#"cursor image."#]
            async fn r#resize(
                client: &mut crate::Client,
                r#seat: crate::wire::ObjectId,
                r#serial: u32,
                r#edges: u32,
            ) -> crate::Result<()>;
            #[doc = r#"Set a maximum size for the window."#]
            #[doc = r#""#]
            #[doc = r#"The client can specify a maximum size so that the compositor does"#]
            #[doc = r#"not try to configure the window beyond this size."#]
            #[doc = r#""#]
            #[doc = r#"The width and height arguments are in window geometry coordinates."#]
            #[doc = r#"See xdg_surface.set_window_geometry."#]
            #[doc = r#""#]
            #[doc = r#"Values set in this way are double-buffered. They will get applied"#]
            #[doc = r#"on the next commit."#]
            #[doc = r#""#]
            #[doc = r#"The compositor can use this information to allow or disallow"#]
            #[doc = r#"different states like maximize or fullscreen and draw accurate"#]
            #[doc = r#"animations."#]
            #[doc = r#""#]
            #[doc = r#"Similarly, a tiling window manager may use this information to"#]
            #[doc = r#"place and resize client windows in a more effective way."#]
            #[doc = r#""#]
            #[doc = r#"The client should not rely on the compositor to obey the maximum"#]
            #[doc = r#"size. The compositor may decide to ignore the values set by the"#]
            #[doc = r#"client and request a larger size."#]
            #[doc = r#""#]
            #[doc = r#"If never set, or a value of zero in the request, means that the"#]
            #[doc = r#"client has no expected maximum size in the given dimension."#]
            #[doc = r#"As a result, a client wishing to reset the maximum size"#]
            #[doc = r#"to an unspecified state can use zero for width and height in the"#]
            #[doc = r#"request."#]
            #[doc = r#""#]
            #[doc = r#"Requesting a maximum size to be smaller than the minimum size of"#]
            #[doc = r#"a surface is illegal and will result in an invalid_size error."#]
            #[doc = r#""#]
            #[doc = r#"The width and height must be greater than or equal to zero. Using"#]
            #[doc = r#"strictly negative values for width or height will result in a"#]
            #[doc = r#"invalid_size error."#]
            async fn r#set_max_size(
                client: &mut crate::Client,
                r#width: i32,
                r#height: i32,
            ) -> crate::Result<()>;
            #[doc = r#"Set a minimum size for the window."#]
            #[doc = r#""#]
            #[doc = r#"The client can specify a minimum size so that the compositor does"#]
            #[doc = r#"not try to configure the window below this size."#]
            #[doc = r#""#]
            #[doc = r#"The width and height arguments are in window geometry coordinates."#]
            #[doc = r#"See xdg_surface.set_window_geometry."#]
            #[doc = r#""#]
            #[doc = r#"Values set in this way are double-buffered. They will get applied"#]
            #[doc = r#"on the next commit."#]
            #[doc = r#""#]
            #[doc = r#"The compositor can use this information to allow or disallow"#]
            #[doc = r#"different states like maximize or fullscreen and draw accurate"#]
            #[doc = r#"animations."#]
            #[doc = r#""#]
            #[doc = r#"Similarly, a tiling window manager may use this information to"#]
            #[doc = r#"place and resize client windows in a more effective way."#]
            #[doc = r#""#]
            #[doc = r#"The client should not rely on the compositor to obey the minimum"#]
            #[doc = r#"size. The compositor may decide to ignore the values set by the"#]
            #[doc = r#"client and request a smaller size."#]
            #[doc = r#""#]
            #[doc = r#"If never set, or a value of zero in the request, means that the"#]
            #[doc = r#"client has no expected minimum size in the given dimension."#]
            #[doc = r#"As a result, a client wishing to reset the minimum size"#]
            #[doc = r#"to an unspecified state can use zero for width and height in the"#]
            #[doc = r#"request."#]
            #[doc = r#""#]
            #[doc = r#"Requesting a minimum size to be larger than the maximum size of"#]
            #[doc = r#"a surface is illegal and will result in an invalid_size error."#]
            #[doc = r#""#]
            #[doc = r#"The width and height must be greater than or equal to zero. Using"#]
            #[doc = r#"strictly negative values for width and height will result in a"#]
            #[doc = r#"invalid_size error."#]
            async fn r#set_min_size(
                client: &mut crate::Client,
                r#width: i32,
                r#height: i32,
            ) -> crate::Result<()>;
            #[doc = r#"Maximize the surface."#]
            #[doc = r#""#]
            #[doc = r#"After requesting that the surface should be maximized, the compositor"#]
            #[doc = r#"will respond by emitting a configure event. Whether this configure"#]
            #[doc = r#"actually sets the window maximized is subject to compositor policies."#]
            #[doc = r#"The client must then update its content, drawing in the configured"#]
            #[doc = r#"state. The client must also acknowledge the configure when committing"#]
            #[doc = r#"the new content (see ack_configure)."#]
            #[doc = r#""#]
            #[doc = r#"It is up to the compositor to decide how and where to maximize the"#]
            #[doc = r#"surface, for example which output and what region of the screen should"#]
            #[doc = r#"be used."#]
            #[doc = r#""#]
            #[doc = r#"If the surface was already maximized, the compositor will still emit"#]
            #[doc = r#"a configure event with the "maximized" state."#]
            #[doc = r#""#]
            #[doc = r#"If the surface is in a fullscreen state, this request has no direct"#]
            #[doc = r#"effect. It may alter the state the surface is returned to when"#]
            #[doc = r#"unmaximized unless overridden by the compositor."#]
            async fn r#set_maximized(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Unmaximize the surface."#]
            #[doc = r#""#]
            #[doc = r#"After requesting that the surface should be unmaximized, the compositor"#]
            #[doc = r#"will respond by emitting a configure event. Whether this actually"#]
            #[doc = r#"un-maximizes the window is subject to compositor policies."#]
            #[doc = r#"If available and applicable, the compositor will include the window"#]
            #[doc = r#"geometry dimensions the window had prior to being maximized in the"#]
            #[doc = r#"configure event. The client must then update its content, drawing it in"#]
            #[doc = r#"the configured state. The client must also acknowledge the configure"#]
            #[doc = r#"when committing the new content (see ack_configure)."#]
            #[doc = r#""#]
            #[doc = r#"It is up to the compositor to position the surface after it was"#]
            #[doc = r#"unmaximized; usually the position the surface had before maximizing, if"#]
            #[doc = r#"applicable."#]
            #[doc = r#""#]
            #[doc = r#"If the surface was already not maximized, the compositor will still"#]
            #[doc = r#"emit a configure event without the "maximized" state."#]
            #[doc = r#""#]
            #[doc = r#"If the surface is in a fullscreen state, this request has no direct"#]
            #[doc = r#"effect. It may alter the state the surface is returned to when"#]
            #[doc = r#"unmaximized unless overridden by the compositor."#]
            async fn r#unset_maximized(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Make the surface fullscreen."#]
            #[doc = r#""#]
            #[doc = r#"After requesting that the surface should be fullscreened, the"#]
            #[doc = r#"compositor will respond by emitting a configure event. Whether the"#]
            #[doc = r#"client is actually put into a fullscreen state is subject to compositor"#]
            #[doc = r#"policies. The client must also acknowledge the configure when"#]
            #[doc = r#"committing the new content (see ack_configure)."#]
            #[doc = r#""#]
            #[doc = r#"The output passed by the request indicates the client's preference as"#]
            #[doc = r#"to which display it should be set fullscreen on. If this value is NULL,"#]
            #[doc = r#"it's up to the compositor to choose which display will be used to map"#]
            #[doc = r#"this surface."#]
            #[doc = r#""#]
            #[doc = r#"If the surface doesn't cover the whole output, the compositor will"#]
            #[doc = r#"position the surface in the center of the output and compensate with"#]
            #[doc = r#"with border fill covering the rest of the output. The content of the"#]
            #[doc = r#"border fill is undefined, but should be assumed to be in some way that"#]
            #[doc = r#"attempts to blend into the surrounding area (e.g. solid black)."#]
            #[doc = r#""#]
            #[doc = r#"If the fullscreened surface is not opaque, the compositor must make"#]
            #[doc = r#"sure that other screen content not part of the same surface tree (made"#]
            #[doc = r#"up of subsurfaces, popups or similarly coupled surfaces) are not"#]
            #[doc = r#"visible below the fullscreened surface."#]
            async fn r#set_fullscreen(
                client: &mut crate::Client,
                r#output: Option<crate::wire::ObjectId>,
            ) -> crate::Result<()>;
            #[doc = r#"Make the surface no longer fullscreen."#]
            #[doc = r#""#]
            #[doc = r#"After requesting that the surface should be unfullscreened, the"#]
            #[doc = r#"compositor will respond by emitting a configure event."#]
            #[doc = r#"Whether this actually removes the fullscreen state of the client is"#]
            #[doc = r#"subject to compositor policies."#]
            #[doc = r#""#]
            #[doc = r#"Making a surface unfullscreen sets states for the surface based on the following:"#]
            #[doc = r#"* the state(s) it may have had before becoming fullscreen"#]
            #[doc = r#"* any state(s) decided by the compositor"#]
            #[doc = r#"* any state(s) requested by the client while the surface was fullscreen"#]
            #[doc = r#""#]
            #[doc = r#"The compositor may include the previous window geometry dimensions in"#]
            #[doc = r#"the configure event, if applicable."#]
            #[doc = r#""#]
            #[doc = r#"The client must also acknowledge the configure when committing the new"#]
            #[doc = r#"content (see ack_configure)."#]
            async fn r#unset_fullscreen(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"Request that the compositor minimize your surface. There is no"#]
            #[doc = r#"way to know if the surface is currently minimized, nor is there"#]
            #[doc = r#"any way to unset minimization on this surface."#]
            #[doc = r#""#]
            #[doc = r#"If you are looking to throttle redrawing when minimized, please"#]
            #[doc = r#"instead use the wl_surface.frame event for this, as this will"#]
            #[doc = r#"also work with live previews on windows in Alt-Tab, Expose or"#]
            #[doc = r#"similar compositor features."#]
            async fn r#set_minimized(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"This configure event asks the client to resize its toplevel surface or"#]
            #[doc = r#"to change its state. The configured state should not be applied"#]
            #[doc = r#"immediately. See xdg_surface.configure for details."#]
            #[doc = r#""#]
            #[doc = r#"The width and height arguments specify a hint to the window"#]
            #[doc = r#"about how its surface should be resized in window geometry"#]
            #[doc = r#"coordinates. See set_window_geometry."#]
            #[doc = r#""#]
            #[doc = r#"If the width or height arguments are zero, it means the client"#]
            #[doc = r#"should decide its own window dimension. This may happen when the"#]
            #[doc = r#"compositor needs to configure the state of the surface but doesn't"#]
            #[doc = r#"have any information about any previous or expected dimension."#]
            #[doc = r#""#]
            #[doc = r#"The states listed in the event specify how the width/height"#]
            #[doc = r#"arguments should be interpreted, and possibly how it should be"#]
            #[doc = r#"drawn."#]
            #[doc = r#""#]
            #[doc = r#"Clients must send an ack_configure in response to this event. See"#]
            #[doc = r#"xdg_surface.configure and xdg_surface.ack_configure for details."#]
            async fn r#configure(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#width: i32,
                r#height: i32,
                r#states: Vec<u8>,
            ) -> crate::Result<()> {
                tracing::debug!("xdg_toplevel -> configure");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_int(width)
                    .put_int(height)
                    .put_array(states)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"The close event is sent by the compositor when the user"#]
            #[doc = r#"wants the surface to be closed. This should be equivalent to"#]
            #[doc = r#"the user clicking the close button in client-side decorations,"#]
            #[doc = r#"if your application has any."#]
            #[doc = r#""#]
            #[doc = r#"This is only a request that the user intends to close the"#]
            #[doc = r#"window. The client may choose to ignore this request, or show"#]
            #[doc = r#"a dialog to ask the user to save their data, etc."#]
            async fn r#close(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("xdg_toplevel -> close");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"The configure_bounds event may be sent prior to a xdg_toplevel.configure"#]
            #[doc = r#"event to communicate the bounds a window geometry size is recommended"#]
            #[doc = r#"to constrain to."#]
            #[doc = r#""#]
            #[doc = r#"The passed width and height are in surface coordinate space. If width"#]
            #[doc = r#"and height are 0, it means bounds is unknown and equivalent to as if no"#]
            #[doc = r#"configure_bounds event was ever sent for this surface."#]
            #[doc = r#""#]
            #[doc = r#"The bounds can for example correspond to the size of a monitor excluding"#]
            #[doc = r#"any panels or other shell components, so that a surface isn't created in"#]
            #[doc = r#"a way that it cannot fit."#]
            #[doc = r#""#]
            #[doc = r#"The bounds may change at any point, and in such a case, a new"#]
            #[doc = r#"xdg_toplevel.configure_bounds will be sent, followed by"#]
            #[doc = r#"xdg_toplevel.configure and xdg_surface.configure."#]
            async fn r#configure_bounds(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#width: i32,
                r#height: i32,
            ) -> crate::Result<()> {
                tracing::debug!("xdg_toplevel -> configure_bounds");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"This event advertises the capabilities supported by the compositor. If"#]
            #[doc = r#"a capability isn't supported, clients should hide or disable the UI"#]
            #[doc = r#"elements that expose this functionality. For instance, if the"#]
            #[doc = r#"compositor doesn't advertise support for minimized toplevels, a button"#]
            #[doc = r#"triggering the set_minimized request should not be displayed."#]
            #[doc = r#""#]
            #[doc = r#"The compositor will ignore requests it doesn't support. For instance,"#]
            #[doc = r#"a compositor which doesn't advertise support for minimized will ignore"#]
            #[doc = r#"set_minimized requests."#]
            #[doc = r#""#]
            #[doc = r#"Compositors must send this event once before the first"#]
            #[doc = r#"xdg_surface.configure event. When the capabilities change, compositors"#]
            #[doc = r#"must send this event again and then send an xdg_surface.configure"#]
            #[doc = r#"event."#]
            #[doc = r#""#]
            #[doc = r#"The configured state should not be applied immediately. See"#]
            #[doc = r#"xdg_surface.configure for details."#]
            #[doc = r#""#]
            #[doc = r#"The capabilities are sent as an array of 32-bit unsigned integers in"#]
            #[doc = r#"native endianness."#]
            async fn r#wm_capabilities(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#capabilities: Vec<u8>,
            ) -> crate::Result<()> {
                tracing::debug!("xdg_toplevel -> wm_capabilities");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_array(capabilities)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 3, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
    pub mod xdg_popup {
        enum r#Error {}
        #[doc = r#"A popup surface is a short-lived, temporary surface. It can be used to"#]
        #[doc = r#"implement for example menus, popovers, tooltips and other similar user"#]
        #[doc = r#"interface concepts."#]
        #[doc = r#""#]
        #[doc = r#"A popup can be made to take an explicit grab. See xdg_popup.grab for"#]
        #[doc = r#"details."#]
        #[doc = r#""#]
        #[doc = r#"When the popup is dismissed, a popup_done event will be sent out, and at"#]
        #[doc = r#"the same time the surface will be unmapped. See the xdg_popup.popup_done"#]
        #[doc = r#"event for details."#]
        #[doc = r#""#]
        #[doc = r#"Explicitly destroying the xdg_popup object will also dismiss the popup and"#]
        #[doc = r#"unmap the surface. Clients that want to dismiss the popup when another"#]
        #[doc = r#"surface of their own is clicked should dismiss the popup using the destroy"#]
        #[doc = r#"request."#]
        #[doc = r#""#]
        #[doc = r#"A newly created xdg_popup will be stacked on top of all previously created"#]
        #[doc = r#"xdg_popup surfaces associated with the same xdg_toplevel."#]
        #[doc = r#""#]
        #[doc = r#"The parent of an xdg_popup must be mapped (see the xdg_surface"#]
        #[doc = r#"description) before the xdg_popup itself."#]
        #[doc = r#""#]
        #[doc = r#"The client must call wl_surface.commit on the corresponding wl_surface"#]
        #[doc = r#"for the xdg_popup state to take effect."#]
        pub trait r#XdgPopup {
            const INTERFACE: &'static str = "xdg_popup";
            const VERSION: u32 = 6;
            async fn handle_request(
                client: &mut crate::Client,
                message: &mut crate::wire::Message,
            ) -> crate::Result<()> {
                match message.opcode {
                    0 => {
                        tracing::debug!("xdg_popup -> destroy");
                        Self::r#destroy(client).await
                    }
                    1 => {
                        tracing::debug!("xdg_popup -> grab");
                        Self::r#grab(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    2 => {
                        tracing::debug!("xdg_popup -> reposition");
                        Self::r#reposition(
                            client,
                            message
                                .object()?
                                .ok_or(crate::wire::DecodeError::MalformedPayload)?,
                            message.uint()?,
                        )
                        .await
                    }
                    _ => Err(crate::error::Error::UnknownOpcode),
                }
            }
            fn create_dispatcher(
                id: crate::wire::ObjectId,
            ) -> std::sync::Arc<Box<dyn crate::Dispatcher + Send + Sync>>;
            #[doc = r#"This destroys the popup. Explicitly destroying the xdg_popup"#]
            #[doc = r#"object will also dismiss the popup, and unmap the surface."#]
            #[doc = r#""#]
            #[doc = r#"If this xdg_popup is not the "topmost" popup, the"#]
            #[doc = r#"xdg_wm_base.not_the_topmost_popup protocol error will be sent."#]
            async fn r#destroy(client: &mut crate::Client) -> crate::Result<()>;
            #[doc = r#"This request makes the created popup take an explicit grab. An explicit"#]
            #[doc = r#"grab will be dismissed when the user dismisses the popup, or when the"#]
            #[doc = r#"client destroys the xdg_popup. This can be done by the user clicking"#]
            #[doc = r#"outside the surface, using the keyboard, or even locking the screen"#]
            #[doc = r#"through closing the lid or a timeout."#]
            #[doc = r#""#]
            #[doc = r#"If the compositor denies the grab, the popup will be immediately"#]
            #[doc = r#"dismissed."#]
            #[doc = r#""#]
            #[doc = r#"This request must be used in response to some sort of user action like a"#]
            #[doc = r#"button press, key press, or touch down event. The serial number of the"#]
            #[doc = r#"event should be passed as 'serial'."#]
            #[doc = r#""#]
            #[doc = r#"The parent of a grabbing popup must either be an xdg_toplevel surface or"#]
            #[doc = r#"another xdg_popup with an explicit grab. If the parent is another"#]
            #[doc = r#"xdg_popup it means that the popups are nested, with this popup now being"#]
            #[doc = r#"the topmost popup."#]
            #[doc = r#""#]
            #[doc = r#"Nested popups must be destroyed in the reverse order they were created"#]
            #[doc = r#"in, e.g. the only popup you are allowed to destroy at all times is the"#]
            #[doc = r#"topmost one."#]
            #[doc = r#""#]
            #[doc = r#"When compositors choose to dismiss a popup, they may dismiss every"#]
            #[doc = r#"nested grabbing popup as well. When a compositor dismisses popups, it"#]
            #[doc = r#"will follow the same dismissing order as required from the client."#]
            #[doc = r#""#]
            #[doc = r#"If the topmost grabbing popup is destroyed, the grab will be returned to"#]
            #[doc = r#"the parent of the popup, if that parent previously had an explicit grab."#]
            #[doc = r#""#]
            #[doc = r#"If the parent is a grabbing popup which has already been dismissed, this"#]
            #[doc = r#"popup will be immediately dismissed. If the parent is a popup that did"#]
            #[doc = r#"not take an explicit grab, an error will be raised."#]
            #[doc = r#""#]
            #[doc = r#"During a popup grab, the client owning the grab will receive pointer"#]
            #[doc = r#"and touch events for all their surfaces as normal (similar to an"#]
            #[doc = r#""owner-events" grab in X11 parlance), while the top most grabbing popup"#]
            #[doc = r#"will always have keyboard focus."#]
            async fn r#grab(
                client: &mut crate::Client,
                r#seat: crate::wire::ObjectId,
                r#serial: u32,
            ) -> crate::Result<()>;
            #[doc = r#"Reposition an already-mapped popup. The popup will be placed given the"#]
            #[doc = r#"details in the passed xdg_positioner object, and a"#]
            #[doc = r#"xdg_popup.repositioned followed by xdg_popup.configure and"#]
            #[doc = r#"xdg_surface.configure will be emitted in response. Any parameters set"#]
            #[doc = r#"by the previous positioner will be discarded."#]
            #[doc = r#""#]
            #[doc = r#"The passed token will be sent in the corresponding"#]
            #[doc = r#"xdg_popup.repositioned event. The new popup position will not take"#]
            #[doc = r#"effect until the corresponding configure event is acknowledged by the"#]
            #[doc = r#"client. See xdg_popup.repositioned for details. The token itself is"#]
            #[doc = r#"opaque, and has no other special meaning."#]
            #[doc = r#""#]
            #[doc = r#"If multiple reposition requests are sent, the compositor may skip all"#]
            #[doc = r#"but the last one."#]
            #[doc = r#""#]
            #[doc = r#"If the popup is repositioned in response to a configure event for its"#]
            #[doc = r#"parent, the client should send an xdg_positioner.set_parent_configure"#]
            #[doc = r#"and possibly an xdg_positioner.set_parent_size request to allow the"#]
            #[doc = r#"compositor to properly constrain the popup."#]
            #[doc = r#""#]
            #[doc = r#"If the popup is repositioned together with a parent that is being"#]
            #[doc = r#"resized, but not in response to a configure event, the client should"#]
            #[doc = r#"send an xdg_positioner.set_parent_size request."#]
            async fn r#reposition(
                client: &mut crate::Client,
                r#positioner: crate::wire::ObjectId,
                r#token: u32,
            ) -> crate::Result<()>;
            #[doc = r#"This event asks the popup surface to configure itself given the"#]
            #[doc = r#"configuration. The configured state should not be applied immediately."#]
            #[doc = r#"See xdg_surface.configure for details."#]
            #[doc = r#""#]
            #[doc = r#"The x and y arguments represent the position the popup was placed at"#]
            #[doc = r#"given the xdg_positioner rule, relative to the upper left corner of the"#]
            #[doc = r#"window geometry of the parent surface."#]
            #[doc = r#""#]
            #[doc = r#"For version 2 or older, the configure event for an xdg_popup is only"#]
            #[doc = r#"ever sent once for the initial configuration. Starting with version 3,"#]
            #[doc = r#"it may be sent again if the popup is setup with an xdg_positioner with"#]
            #[doc = r#"set_reactive requested, or in response to xdg_popup.reposition requests."#]
            async fn r#configure(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#x: i32,
                r#y: i32,
                r#width: i32,
                r#height: i32,
            ) -> crate::Result<()> {
                tracing::debug!("xdg_popup -> configure");
                let payload = crate::wire::PayloadBuilder::new()
                    .put_int(x)
                    .put_int(y)
                    .put_int(width)
                    .put_int(height)
                    .build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 0, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"The popup_done event is sent out when a popup is dismissed by the"#]
            #[doc = r#"compositor. The client should destroy the xdg_popup object at this"#]
            #[doc = r#"point."#]
            async fn r#popup_done(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
            ) -> crate::Result<()> {
                tracing::debug!("xdg_popup -> popup_done");
                let payload = crate::wire::PayloadBuilder::new().build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 1, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
            #[doc = r#"The repositioned event is sent as part of a popup configuration"#]
            #[doc = r#"sequence, together with xdg_popup.configure and lastly"#]
            #[doc = r#"xdg_surface.configure to notify the completion of a reposition request."#]
            #[doc = r#""#]
            #[doc = r#"The repositioned event is to notify about the completion of a"#]
            #[doc = r#"xdg_popup.reposition request. The token argument is the token passed"#]
            #[doc = r#"in the xdg_popup.reposition request."#]
            #[doc = r#""#]
            #[doc = r#"Immediately after this event is emitted, xdg_popup.configure and"#]
            #[doc = r#"xdg_surface.configure will be sent with the updated size and position,"#]
            #[doc = r#"as well as a new configure serial."#]
            #[doc = r#""#]
            #[doc = r#"The client should optionally update the content of the popup, but must"#]
            #[doc = r#"acknowledge the new popup configuration for the new position to take"#]
            #[doc = r#"effect. See xdg_surface.ack_configure for details."#]
            async fn r#repositioned(
                dispatcher_id: crate::wire::ObjectId,
                client: &mut crate::Client,
                r#token: u32,
            ) -> crate::Result<()> {
                tracing::debug!("xdg_popup -> repositioned");
                let payload = crate::wire::PayloadBuilder::new().put_uint(token).build();
                client
                    .send_message(crate::wire::Message::new(dispatcher_id, 2, payload))
                    .await
                    .map_err(crate::error::Error::IoError)
            }
        }
    }
}
