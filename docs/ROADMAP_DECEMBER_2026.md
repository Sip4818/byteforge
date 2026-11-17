# LLM Serving Engine Roadmap (November 17 – December 31, 2026)

## Executive Summary

The objective of this plan is to take you from the existing [`byteforge`](file:///code/projects/byteforge) tokenizer to a **working, production-grade miniature LLM serving engine in Rust** by December 31, 2026.

This project is tailored specifically to prepare you for contributing to **vLLM core** and to serve as a **top-tier resume project for Big Tech AI Systems / ML Infrastructure roles**.

---

## Target Architecture by Dec 31

```
                     ┌──────────────────────────────────┐
                     │     Byteforge Tokenizer          │
                     │  (Text ──► Token IDs ──► Text)   │
                     └────────────────┬─────────────────┘
                                      │
                                      ▼
                     ┌──────────────────────────────────┐
                     │    Async Serving Engine Loop     │
                     └────────────────┬─────────────────┘
                                      │
            ┌─────────────────────────┴─────────────────────────┐
            ▼                                                   ▼
┌────────────────────────┐                             ┌────────────────────────┐
│  Continuous Batching   │                             │     Block Manager      │
│       Scheduler        │                             │  (Paged KV Memory Pool)│
├────────────────────────┤                             ├────────────────────────┤
│ • Waiting / Running /  │                             │ • Physical Block Pool  │
│   Finished Queues      │                             │ • Logical Block Tables │
│ • Prefill / Decode     │                             │ • Reference Counting   │
│   Step Dispatcher      │                             │ • Prefix Cache (Radix) │
└───────────┬────────────┘                             └───────────┬────────────┘
            │                                                      │
            └─────────────────────────┬────────────────────────────┘
                                      ▼
                     ┌──────────────────────────────────┐
                     │    Paged Transformer Forward     │
                     │  (Candle backend / SmolLM /      │
                     │   TinyLlama execution)           │
                     └──────────────────────────────────┘
```

---

## 6-Week Detailed Timeline

### Week 1 (Nov 17 – Nov 23): Bridge Tokenizer & The Naive Baseline
**Objective:** Finalize the tokenizer interface and establish the reference transformer inference baseline.

- [ ] **Byteforge Interface Stabilization:**
  - Expose a clean, thread-safe `Tokenizer` trait / struct.
  - Add basic save/load functionality (JSON serialization of `merges` and `vocab`).
- [ ] **Project Scaffolding (`byteinfer` / `mini-vllm-rs`):**
  - Set up workspace crate or modular module structure.
  - Add `candle-core` and `candle-transformers` as backend dependencies for tensor math (RMSNorm, RoPE, Matmul).
- [ ] **Milestone 1 — Baseline Autoregressive Generation:**
  - Load open model weights (`SmolLM-135M` or `TinyLlama-1.1B`).
  - Run greedy decode step-by-step with naive KV cache (growing tensor).
  - Verify token outputs match Hugging Face reference outputs.

---

### Week 2 (Nov 24 – Nov 30): The `BlockManager` (Virtual Memory for KV Cache)
**Objective:** Implement the core memory management subsystem that powers PagedAttention.

- [ ] **Block Data Structures:**
  - `PhysicalBlockId(usize)`: Unique identifier for GPU/CPU cache chunks.
  - `Block`: Metadata containing allocation status and reference count (`ref_count`).
  - `BlockTable`: Per-sequence array mapping `logical_block_idx -> PhysicalBlockId`.
- [ ] **`BlockManager` Core Implementation:**
  - Implement allocation: request $K$ physical blocks from the free pool.
  - Implement append token logic: if a sequence hits block capacity (e.g. 16 tokens), allocate a new block.
  - Implement deallocation: free blocks when a sequence terminates.
  - Implement Copy-on-Write (CoW): allow multiple sequences to share blocks, incrementing `ref_count`.
- [ ] **Unit Tests & Verification:**
  - Test fragmentation scenario: 100 sequences with random lengths; prove memory utilization is $\ge 96\%$ compared to naive static allocation.

---

### Week 3 (Dec 1 – Dec 7): Paged Attention Execution Layer
**Objective:** Execute the transformer forward pass reading directly from non-contiguous KV blocks.

- [ ] **Paged KV Attention Operator:**
  - Implement or integrate a paged attention lookup:
    Given `query` at token position $t$ and `BlockTable`, gather $K$ and $V$ tensors from the designated physical blocks rather than a single contiguous tensor.
- [ ] **Decode Step with Paged Cache:**
  - Run iterative single-token decode using the `BlockManager`.
- [ ] **Equivalence Testing:**
  - Property test: Ensure `generate(prompt)` with Paged KV produces **bit-exact identical token output** to the naive continuous KV cache.

---

### Week 4 (Dec 8 – Dec 14): Continuous / Iterative Batching Scheduler
**Objective:** Enable multi-request concurrency without head-of-line blocking.

- [ ] **Sequence State Machine:**
  - `Sequence`: `id`, `prompt_tokens`, `output_tokens`, `status` (`Waiting`, `Running`, `Finished`).
- [ ] **The `Scheduler` Loop (vLLM style):**
  - `schedule()`:
    1. Check available free blocks in `BlockManager`.
    2. Promote `Waiting` sequences to `Running` if blocks allow (prefill).
    3. Allocate 1 slot per `Running` sequence for the next decode token.
    4. Free blocks for `Finished` sequences immediately.
- [ ] **Preemption Handling:**
  - If memory is exhausted mid-generation, suspend/swap lowest priority sequence back to `Waiting`.

---

### Week 5 (Dec 15 – Dec 21): End-to-End Pipeline & Byteforge Integration
**Objective:** Wire all components into a running server loop.

- [ ] **End-to-End Inference Loop:**
  - User submits raw text prompt $\rightarrow$ Byteforge tokenizes $\rightarrow$ `Scheduler` queues request $\rightarrow$ `BlockManager` manages KV memory $\rightarrow$ Engine executes forward pass $\rightarrow$ Byteforge decodes and streams tokens back.
- [ ] **Concurrent Serving Test:**
  - Simulate 20 concurrent requests with varying prompt lengths and arrival times.
  - Verify all 20 requests complete accurately without deadlocks or memory leaks.
- [ ] **CI & Tooling:**
  - Ensure `cargo test`, `cargo clippy -D warnings`, and `cargo fmt` pass on GitHub Actions.

---

### Week 6 (Dec 22 – Dec 31): Optimization, Prefix Caching & vLLM Parity
**Objective:** Final high-impact features, benchmarks, and portfolio documentation.

- [ ] **Radix Tree Prefix Caching:**
  - Cache shared system prompts in the `BlockManager`. When two requests share the same prompt prefix, skip prefill and reuse physical blocks directly.
- [ ] **Criterion Benchmarks & Profiling:**
  - Benchmark throughput (tokens/sec) vs concurrency level.
  - Memory consumption graph: Naive KV Cache vs Paged KV Cache.
- [ ] **Resume & vLLM Documentation:**
  - Architecture documentation in `docs/architecture.md`.
  - Detailed README with benchmarks and technical deep-dive into how it mirrors vLLM's internal scheduling and memory paging.

---

## Success Criteria for Dec 31, 2026

1. **A Working Engine:** Capable of loading a real model, ingesting concurrent prompts, and streaming generated text.
2. **True Paged KV:** Memory is managed in discrete blocks with zero external fragmentation.
3. **Continuous Batching:** Requests can join and leave at any iteration step.
4. **All CI Green:** Clean code, `0` clippy warnings, fully covered by automated unit and integration tests.
5. **Direct vLLM Foundation:** Deep familiarity with every concept in `vllm/core/block_manager_v1.py` and `vllm/core/scheduler.py`.
