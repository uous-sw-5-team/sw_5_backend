# Planner Backend

Axum + SurrealDB(RocksDB) 기반 로컬 플래너 API 서버.
**이 문서 하나로 프론트엔드(React) 팀이 바로 연동 개발할 수 있도록 작성되었습니다.**

- 베이스 URL(개발): `http://localhost:8080`
- API prefix: `/api`

## 실행 방법

```bash
cargo run
# → http://localhost:8080
```

DB는 `data/planner.db/` 디렉토리에, 사진은 `uploads/` 디렉토리에 자동 생성/저장됩니다.

## API 문서 (Swagger UI)

서버 실행 후 브라우저에서 아래 주소로 접속하면 모든 엔드포인트를 클릭으로 직접 호출해볼 수 있습니다.

- Swagger UI: `http://localhost:8080/swagger-ui`
- OpenAPI 스펙(JSON): `http://localhost:8080/api-docs/openapi.json`

인증이 필요한 API(플랜/사진/내 정보)는:

1. 먼저 `POST /api/auth/register` 또는 `POST /api/auth/login`을 호출해 응답의 `token` 값을 복사합니다.
2. 우측 상단 **Authorize** 버튼을 눌러 그 토큰을 붙여넣습니다. (앞에 `Bearer ` 안 붙여도 됨)
3. 이후 각 엔드포인트의 **Try it out**으로 호출하면 헤더가 자동으로 들어갑니다.

> 프론트엔드 팀은 `http://localhost:8080/api-docs/openapi.json`을 SwaggerHub나 Swagger Editor, 또는 코드 생성기(openapi-generator)에 그대로 넣어 타입/클라이언트를 자동 생성할 수도 있습니다.

---

## 1. 연동 기본 규약

- 모든 요청/응답은 **JSON** (`Content-Type: application/json`). 단, 사진 업로드만 `multipart/form-data`.
- 인증이 필요한 요청은 **`Authorization: Bearer <token>`** 헤더 필수.
- 토큰은 로그인/회원가입 시 발급되는 **JWT**, 유효기간 **7일**. 만료 시 재로그인.
- 토큰 저장은 **`localStorage`** 사용. 앱 첫 로드 시 저장된 토큰으로 `GET /api/auth/me`를 호출해 세션 복원.
- CORS는 현재 모든 출처 허용(`Any`)이라 로컬 개발에서 별도 설정 불필요.
- 날짜는 항상 `"YYYY-MM-DD"` 문자열. (예: `"2026-06-08"`)
- 에러 응답 형식은 **항상 동일**: `{ "error": "메시지" }` + 적절한 HTTP 상태코드(400/401/404/500).

### 인증 흐름

```
회원가입(register) ─┐
                    ├─→ { token, user } 응답 → token을 localStorage 저장
로그인(login) ──────┘
                          │
        이후 모든 요청 헤더에 Authorization: Bearer <token>
                          │
새로고침/재진입 → localStorage의 token으로 GET /auth/me → 유저 정보 복원
401 응답 → 토큰 만료/무효 → 로그인 화면으로
```

---

## 2. API 명세

### 2-1. 인증

| Method | Path | 인증 | 설명 |
|--------|------|------|------|
| POST | `/api/auth/register` | — | 회원가입 |
| POST | `/api/auth/login` | — | 로그인 (JWT 반환) |
| GET  | `/api/auth/me` | 필요 | 내 정보 |

#### POST `/api/auth/register`
요청
```json
{ "username": "홍길동", "email": "hong@example.com", "password": "mypassword" }
```
응답 `200`
```json
{
  "token": "eyJ...",
  "user": { "id": "user:abc123", "username": "홍길동", "email": "hong@example.com" }
}
```
에러: `400` 이미 사용 중인 이메일.

#### POST `/api/auth/login`
요청
```json
{ "email": "hong@example.com", "password": "mypassword" }
```
응답 `200` — register와 동일한 `{ token, user }` 형태.
에러: `401` 이메일 또는 비밀번호 불일치.

#### GET `/api/auth/me` (인증 필요)
응답 `200`
```json
{ "id": "user:abc123", "username": "홍길동", "email": "hong@example.com" }
```
에러: `401` 토큰 없음/무효.

---

### 2-2. 플랜 (모두 인증 필요)

| Method | Path | 설명 |
|--------|------|------|
| GET    | `/api/plans` | 전체 플랜 목록 |
| GET    | `/api/plans?date=2026-06-08` | 특정 날짜 플랜 |
| GET    | `/api/plans?month=2026-06` | 해당 월 전체 플랜 |
| POST   | `/api/plans` | 플랜 생성 |
| GET    | `/api/plans/{id}` | 플랜 단건 조회 |
| PUT    | `/api/plans/{id}` | 플랜 수정 |
| DELETE | `/api/plans/{id}` | 플랜 삭제 |

> **id 규칙:** 응답의 `plan.id`는 평문 문자열(예: `"x7k2p9"`)입니다.
> `GET/PUT/DELETE /api/plans/{id}` 경로에 이 값을 **그대로** 넣으세요. (접두사 `plan:` 붙이지 않음)

#### GET `/api/plans`
쿼리(선택): `?date=2026-06-08`(특정 날짜), `?month=2026-06`(해당 월). 없으면 전체(최신 날짜순).

응답 `200`
```json
[
  {
    "id": "x7k2p9",
    "user_id": "user:abc123",
    "date": "2026-06-08",
    "title": "역사 에세이 초안 작성",
    "description": "산업혁명의 영향에 관한 보고서 개요",
    "time": "오전 09:00",
    "completed": false,
    "photos": ["a1b2.jpg", "c3d4.png"],
    "created_at": "2026-06-08T01:00:00Z",
    "updated_at": "2026-06-08T01:00:00Z"
  }
]
```

> **필드 메모(프론트 정렬):**
> - `description` — 상세 내용 (프론트의 todo.description)
> - `time` — 표시용 시각 문자열, 자유 형식 (예: `"오전 09:00"`). 없으면 `null`.
> - `completed` — 완료 체크 여부. 생성 시 항상 `false`, 토글은 PUT으로.

#### POST `/api/plans`
요청 (`description`, `time`은 선택)
```json
{ "date": "2026-06-08", "title": "제목", "description": "내용(선택)", "time": "오전 09:00" }
```
응답 `200` — 생성된 plan 객체(위 목록 항목과 동일 구조, `completed`는 `false`).

#### GET `/api/plans/{id}`
응답 `200` plan 객체 / `404` 없음.

#### PUT `/api/plans/{id}`
요청 (모두 선택, 보낸 필드만 반영). 완료 체크 토글도 여기로.
```json
{ "title": "새 제목", "description": "새 내용", "time": "오후 03:00", "completed": true }
```
응답 `200` 수정된 plan 객체.

#### DELETE `/api/plans/{id}`
응답 `200`
```json
{ "message": "삭제 완료" }
```

---

### 2-3. 사진

| Method | Path | 인증 | 설명 |
|--------|------|------|------|
| POST | `/api/plans/{id}/photos` | 필요 | 사진 업로드 (multipart) |
| GET  | `/api/photos/{filename}` | — | 사진 파일 서빙 |

#### POST `/api/plans/{id}/photos` (multipart)
- `multipart/form-data`, 필드명 **`photo`** (여러 개 가능).
- 허용 형식: jpeg / png / webp / gif.
- **주의:** `FormData` 전송 시 `Content-Type` 헤더를 직접 지정하지 마세요. 브라우저가 boundary를 자동 설정합니다.
- 응답 `200` — `photos` 배열이 갱신된 plan 객체.

```bash
curl -X POST http://localhost:8080/api/plans/<plan_id>/photos \
  -H "Authorization: Bearer <token>" \
  -F "photo=@./image.jpg"
```

#### GET `/api/photos/{filename}`
- `plan.photos`의 각 파일명을 그대로 사용.
- 이미지 표시: `<img src="http://localhost:8080/api/photos/a1b2.jpg" />`

---

## 3. React 연동 예시

### 공통 API 래퍼 (`src/api.js`)
```js
const BASE = "http://localhost:8080/api";

function getToken() {
  return localStorage.getItem("token");
}

async function request(path, { method = "GET", body, auth = true } = {}) {
  const headers = {};
  if (body && !(body instanceof FormData)) {
    headers["Content-Type"] = "application/json";
  }
  if (auth) {
    const token = getToken();
    if (token) headers["Authorization"] = `Bearer ${token}`;
  }

  const res = await fetch(`${BASE}${path}`, {
    method,
    headers,
    body: body instanceof FormData ? body : body ? JSON.stringify(body) : undefined,
  });

  if (res.status === 401) {
    localStorage.removeItem("token");
    // 필요 시 로그인 페이지로 redirect
  }

  const data = await res.json().catch(() => ({}));
  if (!res.ok) throw new Error(data.error || `요청 실패 (${res.status})`);
  return data;
}

export const api = {
  // 인증
  register: (payload) => request("/auth/register", { method: "POST", body: payload, auth: false }),
  login:    (payload) => request("/auth/login",    { method: "POST", body: payload, auth: false }),
  me:       () => request("/auth/me"),

  // 플랜
  listPlans: (params = {}) => {
    const qs = new URLSearchParams(params).toString();
    return request(`/plans${qs ? `?${qs}` : ""}`);
  },
  getPlan:    (id) => request(`/plans/${id}`),
  createPlan: (payload) => request("/plans", { method: "POST", body: payload }),
  updatePlan: (id, payload) => request(`/plans/${id}`, { method: "PUT", body: payload }),
  deletePlan: (id) => request(`/plans/${id}`, { method: "DELETE" }),

  // 사진
  uploadPhoto: (planId, file) => {
    const fd = new FormData();
    fd.append("photo", file);
    return request(`/plans/${planId}/photos`, { method: "POST", body: fd });
  },
  photoUrl: (filename) => `${BASE}/photos/${filename}`,
};
```

### 로그인
```jsx
async function handleLogin(email, password) {
  const { token, user } = await api.login({ email, password });
  localStorage.setItem("token", token);
  setUser(user);
}
```

### 세션 복원 (앱 진입 시)
```jsx
useEffect(() => {
  if (!localStorage.getItem("token")) return;
  api.me().then(setUser).catch(() => localStorage.removeItem("token"));
}, []);
```

### 월별 플랜 로드 + 사진 렌더
```jsx
const plans = await api.listPlans({ month: "2026-06" });

// 렌더
{plan.photos.map((f) => (
  <img key={f} src={api.photoUrl(f)} alt="" />
))}
```

### 사진 업로드 (input)
```jsx
async function onFileChange(e, planId) {
  const file = e.target.files[0];
  if (!file) return;
  const updated = await api.uploadPhoto(planId, file); // 갱신된 plan 반환
  setPlan(updated);
}
```

---

## 4. 프론트팀 체크리스트

- 인증 필요한 모든 요청에 `Authorization: Bearer <token>` 빠뜨리지 않기. (누락 시 401)
- 사진 업로드 시 `Content-Type` 수동 설정 금지 (FormData 자동 처리).
- 날짜는 반드시 `"YYYY-MM-DD"` 문자열로 전송. `Date` 객체 그대로 보내지 말 것.
- `plan.id`는 경로에 그대로 사용 (접두사 없음). `user.id`는 `"user:..."` 형태지만 프론트에서 경로로 쓸 일은 없음.
- 에러는 모두 `{ "error": "..." }` 형태 → 래퍼처럼 `data.error`로 메시지 추출.
- 토큰 만료(7일)/무효 시 401 → 로그인 화면으로 유도.

## 5. 백엔드와 합의가 더 필요한 항목

- **배포 시 CORS**: 현재 전체 허용. 운영 환경에서는 프론트 도메인만 허용하도록 좁힐 예정 → 프론트 배포 URL 공유 필요.
- **이미지 인증**: `GET /api/photos/{filename}`은 현재 무인증 접근 가능. 비공개가 필요하면 추후 토큰 검증 추가 예정.
- **페이지네이션**: 플랜 목록은 현재 전량 반환. 데이터가 많아지면 페이징 추가 가능.

---

## 프로젝트 구조 (백엔드)

```
src/
├── main.rs          # 서버 진입점, AppState
├── db.rs            # SurrealDB 연결 (RocksDB 로컬)
├── auth.rs          # JWT 생성/검증, AuthUser extractor
├── error.rs         # AppError 통합 에러 타입
├── models.rs        # User, Plan, PlanPublic, Claims 구조체
└── routes/
    ├── mod.rs       # 라우트 등록
    ├── user.rs      # 회원가입, 로그인, 내 정보
    ├── plan.rs      # 플랜 CRUD
    └── photo.rs     # 사진 업로드/서빙
```
