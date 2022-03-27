const express = require('express')
const axios = require('axios')
const app = express()
const port = 3000

app.get('/', (req, res) => {
    res.send('Hello World!')
})

app.get('/forward', async (req, res) => {
    let url = "http://promoted.hubsvc.itv.com/promoted/itvonline/ctv/homepage/ctv?broadcaster=ITV&features=hls,aes,progressive,inband-webvtt&ishubplus=false&regionalBroadcaster=ITV";
    let accept_header = "application/vnd.itv.hubsvc.promotion.v1+vnd.itv.hubsvc.programme.v3+vnd.itv.hubsvc.production.v3+vnd.itv.hubsvc.channel.v2+hal+json";
    let result;
    result = await axios.get(url, {
        headers: {
            Accept: accept_header
        }
    })
    result = await axios.get(url, {
        headers: {
            Accept: accept_header
        }
    })
    result = await axios.get(url, {
        headers: {
            Accept: accept_header
        }
    })
    result = await axios.get(url, {
        headers: {
            Accept: accept_header
        }
    })
    result = await axios.get(url, {
        headers: {
            Accept: accept_header
        }
    })
    result = await axios.get(url, {
        headers: {
            Accept: accept_header
        }
    })
    result = await axios.get(url, {
        headers: {
            Accept: accept_header
        }
    })
    result = await axios.get(url, {
        headers: {
            Accept: accept_header
        }
    })
    result = await axios.get(url, {
        headers: {
            Accept: accept_header
        }
    })
    result = await axios.get(url, {
        headers: {
            Accept: accept_header
        }
    })
    res.setHeader('Content-Type', 'application/json')
    res.send(result.data)
})
  
app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})
