![townportal icon](images/townportal.gif)

*tp* is a serverless URL shortener based on [Cloudflare Workers](https://developers.cloudflare.com/workers).

> "tp" stands for "Town Portal Scroll" in DotA.

Bilibili Video: *TODO*

## Example

```bash
# 1. Create link
curl -X POST -d '{ "url": "https://google.com" }' https://tp.jerryshell.workers.dev/create
# 2. Get response /goto/cXB3HpG
# 3. Open in browser https://tp.jerryshell.workers.dev/goto/cXB3HpG
```

## Wrangler cmd

### Login

```bash
npx wrangler login
```

### Create KV namespace

```bash
wrangler kv:namespace create tp
wrangler kv:namespace create tp --preview
```

### Dev

```bash
npx wrangler dev
```

### Publish

```bash
npx wrangler publish
```

## LICENSE

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0)
