const socket = new WebSocket('ws://localhost:3000/ws');

socket.addEventListener('open', function (event) {
    socket.send('open');
});

socket.addEventListener('message', function (event) {
    console.log('Message from server ', event.data);
});


setTimeout(() => {
    const obj = { what: "scissors" };
//    const obj = { what: "rock" };
//    const obj = { what: "paper" };

    const blob = new Blob([JSON.stringify(obj, null, 2)], {
      type: "application/json",
    });
    console.log("----------------");
    socket.send(blob);
}, 1000);

setTimeout(() => {
    socket.send('done');
    socket.close(3000, "close");
}, 3000);