// refer code source: https://github.com/redis/node-redis/blob/master/packages/client/lib/commands/index.ts#L9


pub enum RedisCommandRawReply {
    SimpleString(String),
    Integer(i64),
    Buffer(Vec<u8>),
    Null,
    Error(String),
    Array(Vec<RedisCommandRawReply>),
}


pub enum RedisCommandArgument {
    String(String),
    Buffer(Vec<u8>),
}