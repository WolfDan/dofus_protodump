A protoc binary to `.proto` file convertion for Dofus protocol

# Acknowledgement

protoc binary to .proto based on https://github.com/arkadiyt/protodump

.proto reverse reference from https://github.com/LuaxY/dofus-unity-protocol-builder

# Why this over Luaxy's solution

First as a learning experience and second is that the binary definition is closer
to the source of truth than a Json string that's not even documented
on the official csharp docs and can change at any moment

# TODO 

- [ ] parse all binaries and output them correctly
- [ ] CLI flags for in and out
- [ ] CI to automate protoc extraction and .proto file release
