// referred source code: https://github.com/redis/node-redis/blob/master/packages/client/lib/commands/index.ts#L9
// referred source code: https://github.dev/iorust/resp/tree/master/src

/// Represents a RESP value, see [Redis Protocol specification](http://redis.io/topics/protocol).
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum RespValue {
    SimpleString(String), // For Simple Strings the first byte of the reply is "+".
    Integer(i64), // For Integers the first byte of the reply is ":".
    BinaryBulkString(Vec<u8>), // For Bulk <binary> Strings the first byte of the reply is "$".
    BulkString(String), // For Bulk Strings the first byte of the reply is "$".
    Error(String), // For Errors the first byte of the reply is "-".
    Null, // Null bulk reply, `$-1\r\n`
    NullArray, // Null array reply, `*-1\r\n`
    Array(Vec<RespValue>), // For Arrays the first byte of the reply is "*".
}





// impl RespValue {

//     /// Returns `true` if the value is a `Null` or `NullArray`. Returns `false` otherwise.
//     /// # Examples
//     /// ```
//     /// # use self::client::{RespValue};
//     /// assert_eq!(RespValue::Null.is_null(), true);
//     /// assert_eq!(RespValue::NullArray.is_null(), true);
//     /// assert_eq!(RespValue::Integer(123).is_null(), false);
//     /// ```
//     pub fn is_null(&self) -> bool {
//         match *self { 
//             RespValue::Null | RespValue::NullArray => true,
//             _ => false
//         }
//     }

//     /// Returns `true` if the value is a `Error`. Returns `false` otherwise.
//     /// # Examples
//     /// ```
//     /// # use self::client::{RespValue};
//     /// assert_eq!(RespValue::Null.is_error(), false);
//     /// assert_eq!(RespValue::Error("".to_string()).is_error(), true);
//     /// ```
//     pub fn is_error(&self) -> bool {
//         match *self {
//             RespValue::Error(_) => true,
//             _ => false
//         }
//     }

//     // pub fn to_binary_buffer(&self) -> Vec<u8>  {
//     //     encode(self)
//     // }
// }
