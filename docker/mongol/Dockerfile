FROM mongo:7.0


ARG DB_PORT=27017
ARG DB_PATH
ARG LOG_PATH

ENV DB_PORT ${DB_PORT}
ENV DB_PATH ${DB_PATH}
ENV LOG_PATH ${LOG_PATH}

CMD mongod --port ${DB_PORT} --dbpath ${DB_PATH} --logpath ${LOG_PATH} --replSet rs0 --bind_ip 0.0.0.0

HEALTHCHECK --interval=2s --timeout=5s --retries=5 \
    CMD mongosh --eval 'try { rs.status().ok;} catch (e) { rs.initiate(); }'


EXPOSE ${DB_PORT}
