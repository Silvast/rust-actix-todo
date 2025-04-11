FROM postgres:16

# Set environment variables
ENV POSTGRES_PASSWORD=postgres
ENV POSTGRES_USER=postgres
ENV POSTGRES_DB=todos

# Expose PostgreSQL on port 5433
EXPOSE 5433

# Change the default PostgreSQL port from 5432 to 5433
CMD ["postgres", "-c", "port=5433"]
