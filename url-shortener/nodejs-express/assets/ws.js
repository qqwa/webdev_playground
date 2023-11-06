const wsc = new WebSocket('ws://localhost:4000/ws');

wsc.onmessage = (event) => {
    console.log(event);
    const newElement = document.createElement('li')
    const eventList = document.getElementById('list')
    newElement.innerHTML = event.data;
    eventList.prepend(newElement);
};
