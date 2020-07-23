##########
## TEST ##
##########
FROM node:current-slim as tests
RUN mkdir -p ./backend/api-js
COPY ./shared ./shared
COPY ./config/js ./config/js
COPY ./backend/_core/js ./backend/_core/js
COPY ./backend/api-js ./backend/api-js
WORKDIR /backend/api-js
RUN npm install --only=production
CMD [ "npm", "test" ]

#############
## RELEASE ##
#############
FROM node:current-slim as release 
RUN mkdir -p ./backend/api-js
COPY ./shared ./shared
COPY ./config/js ./config/js
COPY ./backend/_core/js ./backend/_core/js
COPY ./backend/api-js ./backend/api-js
WORKDIR /backend/api-js
RUN npm install --only=production
CMD [ "npm", "start" ]

#############
## SANDBOX ##
#############
FROM node:current-slim as sandbox 
RUN mkdir -p ./backend/api-js
COPY ./shared ./shared
COPY ./config/js ./config/js
COPY ./backend/_core/js ./backend/_core/js
COPY ./backend/api-js ./backend/api-js
WORKDIR /backend/api-js
RUN npm install --only=production
CMD [ "npm", "start" ]
