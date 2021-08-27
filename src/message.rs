// https://datatracker.ietf.org/doc/html/rfc2813#section-3.3

// IRC messages are always lines of characters terminated with a CR-LF
//    (Carriage Return - Line Feed) pair, and these messages SHALL NOT
//    exceed 512 characters in length, counting all characters including
//    the trailing CR-LF. Thus, there are 510 characters maximum allowed
//    for the command and its parameters.  There is no provision for
//    continuation message lines.

struct Message {}
