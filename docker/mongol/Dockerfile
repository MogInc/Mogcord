FROM mongo:7.0

ARG IS_CHILD=""
ARG REPL_CHILDREN=""
ARG CONTAINER_NAME
ARG DB_PORT=27017
ARG DB_PATH
ARG LOG_PATH

ENV IS_CHILD=${IS_CHILD}
ENV REPL_CHILDREN=${REPL_CHILDREN}
ENV CONTAINER_NAME=${CONTAINER_NAME}
ENV DB_PORT=${DB_PORT}
ENV DB_PATH=${DB_PATH}
ENV LOG_PATH=${LOG_PATH}

CMD mongod --port ${DB_PORT} --dbpath ${DB_PATH} --logpath ${LOG_PATH} --replSet rs0 --bind_ip 0.0.0.0

HEALTHCHECK --interval=2s --timeout=5s --retries=5 \
    CMD mongosh --eval "try { if(!process.env.IS_CHILD){rs.status().ok;} } catch (e) { let counter = 0; let config = [{_id: counter++, host: process.env.CONTAINER_NAME.concat(':', process.env.DB_PORT)}]; process.env.REPL_CHILDREN.split(',').filter(String).map(x => x.trim()).forEach(x => config.push({_id: counter++, host: x.concat(':', process.env.DB_PORT)})); rs.initiate({ _id: 'rs0', members: config}).ok;}"

EXPOSE ${DB_PORT}
