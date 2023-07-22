const websocket = @import("websocket");
const Client = websocket.Client;
const Message = websocket.Message;
const Handshake = websocket.Handshake;

// Define a struct for "global" data passed into your websocket handler
const Context = struct {};

const Handler = struct {
    client: *Client,
    context: *Context,

    pub fn init(h: Handshake, client: *Client, context: *Context) !Handler {
        // `h` contains the initial websocket "handshake" request
        // It can be used to apply application-specific logic to verify / allow
        // the connection (e.g. valid url, query string parameters, or headers)

        _ = h; // we're not using this in our simple case

        return Handler{
            .client = client,
            .context = context,
        };
    }

    // optional hook that, if present, will be called after initialization is complete
    pub fn afterInit(self: *Handler) !void {
        _ = self;
    }

    pub fn handle(self: *Handler, message: Message) !void {
        const data = message.data;
        try self.client.write(data); // echo the message back
    }

    // called whenever the connection is closed, can do some cleanup in here
    pub fn close(_: *Handler) void {}
};
