const ws = new WebSocket('ws://0.0.0.0:5002');

ws.onopen = () => {
    console.log("Connected to WebSocket server");
};

ws.onmessage = (event) => {
    document.getElementById('responseBox').innerText = 'Received: ' + event.data;
};

function sendMessage(min, max, iter) { //put obj here
    const message = JSON.stringify({min,max,iter})
    console.log(message)
    ws.send(message);
}

const form = document.getElementById('numberForm');
form.addEventListener('submit', function(e) {
    e.preventDefault();
    const min = parseFloat(form.min.value);
    const max = parseFloat(form.max.value);
    const iter = parseFloat(form.iter.value);

    if (form.action === "submit") {
        console.log("Submit button clicked");
        if (max < min) {
            alert('Max cannot be less than Min.');
        }
        else{
            sendMessage(min,max,iter);
        }
    } else if (form.action === "end") {
        console.log("End button clicked");
        min = 0;
        max = 0;
        iter = 1;
        sendMessage(min,max,iter);
    }
    
})
