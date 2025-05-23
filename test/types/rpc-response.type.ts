export type RpcResponse = RpcResponseSuccess | RpcResponseError;

export type RpcResponseSuccess = {
    jsonrpc: "2.0";
    id: number;
    result: unknown;
};
export type RpcResponseError = {
    jsonrpc: "2.0";
    id: number;
    error: {
        code: number;
        message: string;
    };
};
