# Planner Backend

Axum + SurrealDB(RocksDB) 기반 로컬 플래너 API 서버

## 실행 방법

```bash
cargo run
# → http://localhost:8080
```

DB는 `data/planner.db/` 디렉토리에 자동 생성됨.  
사진은 `uploads/` 디렉토리에 저장됨.

---

## API 목록

### 인증

| Method | Path | 설명 |
|--------|------|------|
| POST | `/api/auth/register` | 회원가입 |
| POST | `/api/auth/login` | 로그인 (JWT 반환) |
| GET  | `/api/auth/me` | 내 정보 (인증 필요) |

**회원가입/로그인 예시**
```json
POST /api/auth/register
{
  "username": "홍길동",
  "email": "hong@example.com",
  "password": "mypassword"
}

→ { "token": "eyJ...", "user": { "id": "...", "username": "홍길동", "email": "..." } }
```

이후 모든 인증 필요 요청에 헤더 추가:
```
Authorization: Bearer <token>
```

---

### 플랜

| Method | Path | 설명 |
|--------|------|------|
| GET    | `/api/plans` | 전체 플랜 목록 |
| GET    | `/api/plans?date=2025-06-07` | 특정 날짜 플랜 |
| GET    | `/api/plans?month=2025-06` | 해당 월 전체 플랜 |
| POST   | `/api/plans` | 플랜 생성 |
| GET    | `/api/plans/:id` | 플랜 단건 조회 |
| PUT    | `/api/plans/:id` | 플랜 수정 |
| DELETE | `/api/plans/:id` | 플랜 삭제 |

**플랜 생성 예시**
```json
POST /api/plans
{
  "date": "2025-06-07",
  "title": "제주도 여행 첫째 날",
  "content": "한라산 등반 예정"
}
```

---

### 사진

| Method | Path | 설명 |
|--------|------|------|
| POST | `/api/plans/:id/photos` | 사진 업로드 (multipart) |
| GET  | `/api/photos/:filename` | 사진 파일 서빙 |

**사진 업로드**
```bash
curl -X POST http://localhost:8080/api/plans/<plan_id>/photos \
  -H "Authorization: Bearer <token>" \
  -F "photo=@./image.jpg"
```

응답으로 해당 플랜의 `photos` 배열에 파일명이 추가됨.  
`GET /api/photos/<filename>` 으로 이미지 직접 접근 가능.

---

## 프로젝트 구조

```
src/
├── main.rs          # 서버 진입점, AppState
├── db.rs            # SurrealDB 연결 (RocksDB 로컬)
├── auth.rs          # JWT 생성/검증, AuthUser extractor
├── error.rs         # AppError 통합 에러 타입
├── models.rs        # User, Plan, Claims 구조체
└── routes/
    ├── mod.rs       # 라우트 등록
    ├── user.rs      # 회원가입, 로그인, 내 정보
    ├── plan.rs      # 플랜 CRUD
    └── photo.rs     # 사진 업로드/서빙
```
