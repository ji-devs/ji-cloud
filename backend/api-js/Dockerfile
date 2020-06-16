##########
## TEST ##
##########
FROM node:latest-slim as tests
WORKDIR /usr/src/app
COPY package*.json ./
RUN npm install --only=production
COPY . .
CMD [ "npm", "test" ]

#############
## RELEASE ##
#############
FROM node:latest-slim as release 
WORKDIR /usr/src/app
COPY package*.json ./
RUN npm install --only=production
COPY . .
CMD [ "npm", "start" ]

#############
## SANDBOX ##
#############
FROM node:latest-slim as sandbox 
WORKDIR /usr/src/app
COPY package*.json ./
RUN npm install --only=production
COPY . .
CMD [ "npm", "start" ]
