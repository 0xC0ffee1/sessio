package net.c0ffee1.sessio.ui

class GrpcServer {
    // Load the native library
    init {
        System.loadLibrary("grpc_server")
    }

    // Declare the native method
    external fun start_grpc_server(path: String)
}