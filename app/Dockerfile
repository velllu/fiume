# Not using node:alpine3.19 shaves off like 100 mbs
FROM alpine:3.19
WORKDIR /usr/src/fiume-app
COPY . .
RUN apk add npm
RUN npm install
RUN npx nuxi@latest build
ENV API_ADDRESS=api
CMD ["node", ".output/server/index.mjs"]
