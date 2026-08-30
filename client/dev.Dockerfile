FROM node:24-alpine

ARG PNPM_VERSION=11.9.0

WORKDIR /app/client

RUN corepack enable \
    && corepack prepare pnpm@${PNPM_VERSION} --activate \
    && pnpm --version

COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

EXPOSE 3000

CMD ["sh", "-c", "pnpm install --frozen-lockfile && pnpm dev --host 0.0.0.0"]
