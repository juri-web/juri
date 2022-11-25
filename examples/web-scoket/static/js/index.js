// https://zh.javascript.info/websocket

let socket = new WebSocket(
    "ws://127.0.0.2:8080/ws"
);

socket.onopen = function (e) {
    alert("[open] Connection established");
    alert("Sending to server");
    socket.send("My name is John");
};

socket.onmessage = function (event) {
    alert(`[message] Data received from server: ${event.data}`);
};

socket.onclose = function (event) {
    if (event.wasClean) {
        alert(
            `[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`
        );
    } else {
        // 例如服务器进程被杀死或网络中断
        // 在这种情况下，event.code 通常为 1006
        alert("[close] Connection died");
    }
};

socket.onerror = function (error) {
    alert(`[error] ${error.message}`);
};
