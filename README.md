# Github to Discord Webhook Forwarder

This is a [Cloudflare Worker](https://workers.cloudflare.com/) that forwards Github webhooks to a Discord webhook.

## Worker setup

Clone this repository and install the dependencies with `npm install`.

(Optional) Set the right `REVIEWERS_ROLE_ID` and `GH_REVIEWER_TEAM` in `wrangler.toml`.

Set the `GH_SECRET` secret to a random string by running

```shell
echo random_string_here | wrangler secret put GH_SECRET
```

Set the `WEBHOOK_URL` secret by running

```shell
echo https://discord.com/api/webhooks/... | wrangler secret put WEBHOOK_URL
```

Run `wrangler deploy` to deploy the worker.

## Github Setup

Create a new webhook in your Github repository settings.

![Create Github Webhook](tutorial/1.png)

Put in the URL of your worker, select `application/json` as the content type and use a the random secret you set earlier.

![Github Webhook settings](tutorial/2.png)

Select "Let me select individual events" and check "Pull requests".

![Github Webhook settings](tutorial/3.png)

![Github Webhook settings](tutorial/4.png)

After that, press "Add webhook" and you're done!
