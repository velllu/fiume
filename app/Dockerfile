FROM node:alpine3.19
WORKDIR /usr/src/fiume-app
COPY . .
RUN npm install
RUN npx nuxi@latest build
ENV API_ADDRESS=api
CMD ["node", ".output/server/index.mjs"]
