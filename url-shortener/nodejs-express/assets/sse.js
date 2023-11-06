const eventSource = new EventSource("/sse");

eventSource.onmessage = (event) => {
    const newElement = document.createElement('li')
    const eventList = document.getElementById('list')
    newElement.innerHTML = event.data;
    eventList.prepend(newElement);
}
