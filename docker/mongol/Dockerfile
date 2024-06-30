FROM mongo:7.0

ARG DB_PATH
ARG LOG_PATH
ARG REP_NAME

ENV DB_PATH ${DB_PATH}
ENV LOG_PATH ${LOG_PATH}
ENV REP_NAME ${REP_NAME}

CMD mongod --port 27017 --dbpath ${DB_PATH} --logpath ${LOG_PATH} --replSet ${REP_NAME} --bind_ip 0.0.0.0

# 
# HEALTHCHECK --interval=2s --timeout=5s --retries=5 \
# CMD mongosh --eval "try { rs.status().ok } catch (e) { rs.initiate({ _id: '${REP_NAME}', members: [{ _id: 0, host: '127.0.0.1:27017' }] }).ok }"

HEALTHCHECK --interval=2s --timeout=5s --retries=5 \
    CMD mongosh --eval 'rs.initiate()'


EXPOSE 27017
