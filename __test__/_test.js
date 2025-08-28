import { Window } from '../index.js'

const window = new Window({
  devtools: true,
})
window.setTitle('Glacier App')

const code = `
<html>
    <body>
        <h1>Glacier</h1>
        <p>Hello from Glacier</p>
        <pre>Node.js version: <code>${process.versions.node}</code></pre>
        <pre>V8 version: <code>${process.versions.v8}</code></pre>
        <pre>webview version: <code>${Window.getWebviewVersion()}</code></pre>
        <pre>glacier lib version: <code>${Window.getLibVersion()}</code></pre>
        <script>
            window.ipc.postMessage('ping');
        </script>
    </body>
</html>
`
window.loadHtml(code)

function ipcHandler(data) {
  console.log(`IPC data: ${data}`)
}

window.create(ipcHandler)
