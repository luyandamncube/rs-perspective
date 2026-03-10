// server-rs\static\streaming.js
import "https://cdn.jsdelivr.net/npm/@perspective-dev/viewer@4.2.0/dist/cdn/perspective-viewer.js";
import "https://cdn.jsdelivr.net/npm/@perspective-dev/viewer-datagrid@4.2.0/dist/cdn/perspective-viewer-datagrid.js";
import "https://cdn.jsdelivr.net/npm/@perspective-dev/viewer-d3fc@4.2.0/dist/cdn/perspective-viewer-d3fc.js";

import perspective from "https://cdn.jsdelivr.net/npm/@perspective-dev/client@4.2.0/dist/cdn/perspective.js";

const elem = document.getElementsByTagName("perspective-viewer")[0];

function log(...args) {
    console.log("[streaming.js]", ...args);
}

const protocol = window.location.protocol === "https:" ? "wss" : "ws";
const wsUrl = `${protocol}://${window.location.host}/ws`;

log("connecting to websocket", wsUrl);

const client = await perspective.websocket(wsUrl);
log("websocket connected");

const tableNames = await client.get_hosted_table_names();
log("hosted tables", tableNames);

await elem.load(client);
log("viewer loaded client");

await elem.restore({
    plugin: "Datagrid",
    plugin_config: {
        editable: false,
        scroll_lock: true,
    },
    settings: true,
    table: "ticks",
    theme: "Pro Light",
    group_by: ["name"],
    split_by: ["client"],
    columns: ["chg", "bid", "ask", "vol"],
    sort: [["chg", "desc"]],
});

log("viewer restore complete");