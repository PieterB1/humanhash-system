FROM node:18
WORKDIR /app
COPY package.json .
RUN npm install
COPY . .
bash
CMD ["npm", "start"]
