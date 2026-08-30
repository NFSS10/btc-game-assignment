import { beforeEach, describe, expect, it, vi } from "vitest";

import { get, post } from "./utils";

describe("api utils", () => {
    /**
     * Runs before every test.
     * We reset all mocked/stubbed globals to avoid one test affecting another.
     */
    beforeEach(() => {
        vi.restoreAllMocks();
        vi.unstubAllGlobals();
    });

    it("get: returns json on success", async () => {
        // fake `fetch` that resolves to a successful response-like object.
        const fetchMock = vi.fn().mockResolvedValue({
            ok: true,
            json: async () => ({ ok: true })
        });
        vi.stubGlobal("fetch", fetchMock);

        const result = await get("/my/endpoint");

        expect(result).toEqual({ ok: true });
        expect(fetchMock).toHaveBeenCalledOnce();
    });

    it("get: throws on non-ok response", async () => {
        // fake failed HTTP response.
        const fetchMock = vi.fn().mockResolvedValue({
            ok: false,
            statusText: "Not Found",
            json: async () => ({})
        });
        vi.stubGlobal("fetch", fetchMock);

        const promise = get("/missing");

        // expect rejection with precise error text
        await expect(promise).rejects.toThrow("Failed to fetch /missing: Not Found");
    });

    it("get: throws when success body is not valid json", async () => {
        const fetchMock = vi.fn().mockResolvedValue({
            ok: true,
            json: async () => {
                throw new SyntaxError("Unexpected token < in JSON");
            }
        });
        vi.stubGlobal("fetch", fetchMock);

        const result = get("/html-endpoint");
        expect(result).rejects.toThrow(SyntaxError);
    });

    it("post: sends json body and returns response json", async () => {
        // fake success response for POST.
        const fetchMock = vi.fn().mockResolvedValue({
            ok: true,
            json: async () => ({ created: true })
        });
        vi.stubGlobal("fetch", fetchMock);

        const body = { name: "alice" };
        const result = await post("/users", body);

        expect(result).toEqual({ created: true });
        expect(fetchMock).toHaveBeenCalledOnce();

        // ensure request is correctly configured by our helper.
        const [, reqInit] = fetchMock.mock.calls[0];
        expect(reqInit).toMatchObject({
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(body)
        });
    });

    it("post: throws on non-ok response", async () => {
        // fake failed response for POST.
        const fetchMock = vi.fn().mockResolvedValue({
            ok: false,
            statusText: "Bad Request",
            json: async () => ({})
        });
        vi.stubGlobal("fetch", fetchMock);

        const promise = post("/users", { x: 1 });

        // expect rejection with precise error text.
        await expect(promise).rejects.toThrow("Failed to post /users: Bad Request");
    });

    it("post: throws when success body is not valid json", async () => {
        const fetchMock = vi.fn().mockResolvedValue({
            ok: true,
            json: async () => {
                throw new SyntaxError("Unexpected token < in JSON");
            }
        });
        vi.stubGlobal("fetch", fetchMock);

        const result = post("/text-endpoint", { a: 1 });

        await expect(result).rejects.toThrow(SyntaxError);
    });

    it("post: throws if body serialization fails", async () => {
        // this should never be called
        const fetchMock = vi.fn();
        vi.stubGlobal("fetch", fetchMock);

        // create circular object to force JSON.stringify failure.
        const circular: Record<string, unknown> = {};
        circular.self = circular;

        const result = post("/stuff", circular);

        // expect immediate error during body serialization step.
        await expect(result).rejects.toThrow("Failed to serialize request body for /stuff");

        // ensure no network call happened
        expect(fetchMock).not.toHaveBeenCalled();
    });
});
