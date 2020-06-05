##########
## TEST ##
##########
FROM node:12-slim as tests
WORKDIR /usr/src/app
COPY package*.json ./
RUN npm install --only=production
COPY . .
CMD [ "npm", "test" ]

#############
## RELEASE ##
#############
FROM node:12-slim as release 
WORKDIR /usr/src/app
COPY package*.json ./
RUN npm install --only=production
COPY . .
CMD [ "npm", "start" ]

#############
## SANDBOX ##
#############
FROM node:12-slim as sandbox 
WORKDIR /usr/src/app
COPY package*.json ./
RUN npm install --only=production
COPY . .
CMD [ "npm", "run", "start:sandbox" ]
