package net.c0ffee1.sessio.ui

import App
import android.content.pm.PackageManager
import android.net.LocalSocket
import android.net.LocalSocketAddress
import android.os.Build
import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.annotation.RequiresApi
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Button
import androidx.compose.material.MaterialTheme
import androidx.compose.material.OutlinedTextField
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import androidx.lifecycle.lifecycleScope
import com.google.protobuf.kotlin.toByteStringUtf8
import io.grpc.ManagedChannelBuilder
import io.grpc.android.UdsChannelBuilder
import io.grpc.examples.helloworld.GreeterGrpcKt
import io.grpc.examples.helloworld.helloRequest
import java.io.File
import java.nio.ByteBuffer
import java.nio.channels.SocketChannel
import io.grpc.okhttp.OkHttpChannelBuilder
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.asExecutor
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.filter
import kotlinx.coroutines.flow.flatMapConcat
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.onCompletion
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import net.c0ffee1.sessio.clientipc.ClientIPCGrpcKt
import net.c0ffee1.sessio.clientipc.GenKeysRequest
import net.c0ffee1.sessio.clientipc.Msg
import net.c0ffee1.sessio.clientipc.Msg.Data
import net.c0ffee1.sessio.clientipc.NewConnectionRequest
import net.c0ffee1.sessio.clientipc.NewSessionRequest
import org.jetbrains.compose.resources.stringResource
import java.io.Closeable


class GreeterRCP(path: String) : Closeable {
    val responseState = mutableStateOf("")
    val inputState = MutableStateFlow("")

    private val channel = let {
        println("Connecting to ${path}")

        val builder = UdsChannelBuilder.forPath(path, LocalSocketAddress.Namespace.FILESYSTEM)

        builder.executor(Dispatchers.IO.asExecutor()).build()
    }

    private val _ipc = ClientIPCGrpcKt.ClientIPCCoroutineStub(channel)
    val ipc: ClientIPCGrpcKt.ClientIPCCoroutineStub
        get() = _ipc

    suspend fun newShell(privateKeyPath: String) {
        try {
            val connectionId = ipc.newConnection(NewConnectionRequest.newBuilder()
                .setTargetId("vm")
                .setCoordinatorUrl("quic://157.90.127.19:2223")
                .build()).connectionId

            val sessionId = ipc.newSession(NewSessionRequest.newBuilder()
                .setUsername("test")
                .setPrivateKey(privateKeyPath)
                .setConnectionId(connectionId)
                .build()).sessionId;

            //Open the channel
            val initialFlow = flow {
                //Open channel
                emit(
                    Msg.newBuilder().setChannelInit(
                        Msg.ChannelInit.newBuilder().setSessionId(sessionId).build()
                    ).build()
                )

                //Request PTY
                emit(
                    Msg.newBuilder().setPtyRequest(
                        Msg.PtyRequest.newBuilder()
                            .setColWidth(40)
                            .setRowHeight(0)
                            .build()
                    ).build()
                )

                //Request shell, starts bidirectional stream
                emit(
                    Msg.newBuilder().setShellRequest(
                        Msg.ShellRequest.newBuilder().build()
                    ).build()
                )
            }

            val requestFlow = initialFlow.onCompletion {
                inputState.collect { input ->
                    Msg.newBuilder().setData(Data.newBuilder().setPayload(input.toByteStringUtf8())).build()
                }
            }

            // Send the flow to the server
            val responseFlow = ipc.openChannel(requestFlow)

            // Collect the responses and update the response state
            responseFlow.collect { response ->
                when(response.typeCase) {
                    Msg.TypeCase.DATA -> {
                        responseState.value = response.data.payload.toStringUtf8()
                    }

                    Msg.TypeCase.PTY_REQUEST -> TODO()
                    Msg.TypeCase.SHELL_REQUEST -> TODO()
                    Msg.TypeCase.CHANNEL_INIT -> TODO()
                    Msg.TypeCase.TYPE_NOT_SET -> TODO()
                }
            }
        } catch (e: Exception) {
            responseState.value = e.message ?: "Unknown Error"
            e.printStackTrace()
        }
    }

    override fun close() {
        channel.shutdownNow()
    }
}

@Composable
fun Greeter(greeterRCP: GreeterRCP, privateKeyPath: String) {
    val scope = rememberCoroutineScope()
    val nameState = remember { mutableStateOf(TextFieldValue()) }

    Column(
        modifier = Modifier.fillMaxWidth().fillMaxHeight(),
        verticalArrangement = Arrangement.Top,
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Text("Name:", modifier = Modifier.padding(top = 10.dp))
        OutlinedTextField(
            value = nameState.value,
            onValueChange = {
                nameState.value = it
                greeterRCP.inputState.value = it.text
            }
        )

        Button(
            onClick = { scope.launch { greeterRCP.newShell(privateKeyPath) } },
            modifier = Modifier.padding(10.dp)
        ) {
            Text("Start gRPC Communication")
        }

        if (greeterRCP.responseState.value.isNotEmpty()) {
            Text("Server response:", modifier = Modifier.padding(top = 10.dp))
            Text(greeterRCP.responseState.value)
        }
    }
}

class MainActivity : ComponentActivity() {


    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val socketPath = this.filesDir.absolutePath + "/mysocket6"
        val grpcServer = GrpcServer()



        grpcServer.start_grpc_server(socketPath)
        Log.i("MainActivity", "called grpc start")

        lifecycleScope.launch {

            val greeterService = GreeterRCP(socketPath)

            // Await the genKeys function call
            withContext(Dispatchers.IO) {
                greeterService.ipc.genKeys(
                    GenKeysRequest.newBuilder()
                        .setKeyPath(this@MainActivity.filesDir.absolutePath)
                        .build()
                )
            }

            // Set the content of the activity after genKeys completes
            setContent {
                Surface(color = MaterialTheme.colors.background) {
                    Greeter(greeterService, "your-session-id") // Pass the appropriate session ID
                }
            }
        }
    }


}

@Preview
@Composable
fun AppAndroidPreview() {
    App()
}