use api_types::{ApiResponse, PaginationMeta};
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_api_response_success(c: &mut Criterion) {
    c.bench_function("api_response_success", |b| {
        b.iter(|| {
            let response = ApiResponse::success("hello");
            std::hint::black_box(response);
        });
    });
}

fn bench_api_response_error(c: &mut Criterion) {
    c.bench_function("api_response_error", |b| {
        b.iter(|| {
            let response = ApiResponse::<()>::error_with("NOT_FOUND", "Resource not found");
            std::hint::black_box(response);
        });
    });
}

fn bench_api_response_from(c: &mut Criterion) {
    c.bench_function("api_response_from", |b| {
        b.iter(|| {
            let response: ApiResponse<i32> = ApiResponse::from(42);
            std::hint::black_box(response);
        });
    });
}

fn bench_api_response_paginated(c: &mut Criterion) {
    let meta = PaginationMeta {
        page: 1,
        per_page: 20,
        total: 1000,
        total_pages: 50,
    };
    c.bench_function("api_response_paginated", |b| {
        b.iter(|| {
            let response = ApiResponse::paginated(vec![1, 2, 3], meta.clone());
            std::hint::black_box(response);
        });
    });
}

fn bench_api_response_to_json(c: &mut Criterion) {
    let response = ApiResponse::success("hello world");
    c.bench_function("api_response_to_json", |b| {
        b.iter(|| {
            let json = serde_json::to_string(&response).unwrap();
            std::hint::black_box(json);
        });
    });
}

fn bench_api_response_to_json_pretty(c: &mut Criterion) {
    let response = ApiResponse::success("hello world");
    c.bench_function("api_response_to_json_pretty", |b| {
        b.iter(|| {
            let json = serde_json::to_string_pretty(&response).unwrap();
            std::hint::black_box(json);
        });
    });
}

fn bench_api_error_response_to_json(c: &mut Criterion) {
    let response = ApiResponse::<()>::error_with("VALIDATION_ERROR", "name is required");
    c.bench_function("api_error_response_to_json", |b| {
        b.iter(|| {
            let json = serde_json::to_string(&response).unwrap();
            std::hint::black_box(json);
        });
    });
}

fn bench_pagination_meta_creation(c: &mut Criterion) {
    c.bench_function("pagination_meta_creation", |b| {
        b.iter(|| {
            let meta = PaginationMeta {
                page: 1,
                per_page: 20,
                total: 1000,
                total_pages: 50,
            };
            std::hint::black_box(meta);
        });
    });
}

criterion_group!(
    benches,
    bench_api_response_success,
    bench_api_response_error,
    bench_api_response_from,
    bench_api_response_paginated,
    bench_api_response_to_json,
    bench_api_response_to_json_pretty,
    bench_api_error_response_to_json,
    bench_pagination_meta_creation,
);
criterion_main!(benches);
