# 프론트엔드(React) API 연동 가이드

> Planner 백엔드(Axum + SurrealDB) 연동 문서. 프론트엔드 팀 전달용.
> 백엔드 베이스 URL: `http://localhost:8080` (개발 기준)

---

## 1. 기본 규약

- 모든 요청/응답은 **JSON** (`Content-Type: application/json`). 단, 사진 업로드만 `multipart/form-data`.
- 인증이 필요한 요청은 **`Authorization: Bearer <token>`** 헤더 필수.
- 토큰은 로그인/회원가입 시 1회 발급되는 **JWT**, 유효기간 **7일**. 만료되면 재로그인.
- CORS는 현재 모든 출처 허용(`Any`)이라 로컬 개발에서 별도 설정 불필요.
- 날짜는 항상 `"YYYY-MM-DD"` 문자열. (예: `"2026-06-08"`)
- 에러 응답 형식은 **항상 동일**: `{ "error": "메시지" }` + 적절한 HTTP 상태코드.

### 토큰 저장
이번 합의안은 **`localStorage`** 사용. 로그인 성공 시 `localStorage.setItem('token', token)`, 로그아웃 시 `removeItem`. 앱 첫 로드 시 `localStorage`의 토큰으로 `GET /api/auth/me`를 호출해 세션 복원.

---

## 2. 인증 흐름

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

## 3. 엔드포인트 명세

### 인증

#### POST `/api/auth/register` — 회원가입
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

#### POST `/api/auth/login` — 로그인
요청
```json
{ "email": "hong@example.com", "password": "mypassword" }
```
응답 `200` — register와 동일한 `{ token, user }` 형태.
에러: `401` 이메일 또는 비밀번호 불일치.

#### GET `/api/auth/me` — 내 정보 (인증 필요)
응답 `200`
```json
{ "id": "user:abc123", "username": "홍길동", "email": "hong@example.com" }
```
에러: `401` 토큰 없음/무효.

---

### 플랜 (모두 인증 필요)

> **id 규칙:** 응답의 `plan.id`는 평문 문자열(예: `"x7k2p9"`)입니다.
> `GET/PUT/DELETE /api/plans/{id}` 경로에 이 값을 **그대로** 넣으면 됩니다. (접두사 `plan:` 붙이지 마세요)

#### GET `/api/plans` — 목록
쿼리(선택):
- `?date=2026-06-08` — 특정 날짜
- `?month=2026-06` — 해당 월 전체
- 없으면 전체(최신 날짜순)

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

> **프론트 Todo ↔ 백엔드 Plan 필드 매핑**
> | 프론트 | 백엔드 | 비고 |
> |---|---|---|
> | `title` | `title` | |
> | `description` | `description` | |
> | `time` | `time` | 자유 형식 문자열 (예: `"오전 09:00"`), 없으면 `null` |
> | `completed` | `completed` | 생성 시 항상 `false`, 토글은 PUT |
> | `date` | `date` | `"YYYY-MM-DD"` |
> | `id` (number) | `id` (string) | 프론트는 string으로 받도록 변경 필요 |

#### POST `/api/plans` — 생성
요청 (`description`, `time`은 선택)
```json
{ "date": "2026-06-08", "title": "제목", "description": "내용(선택)", "time": "오전 09:00" }
```
응답 `200` — 생성된 plan 객체 (위 목록 항목과 동일 구조, `completed`는 `false`).

#### GET `/api/plans/{id}` — 단건 조회
응답 `200` plan 객체 / `404` 없음.

#### PUT `/api/plans/{id}` — 수정 (완료 체크 토글 포함)
요청 (모두 선택, 보낸 필드만 반영)
```json
{ "title": "새 제목", "description": "새 내용", "time": "오후 03:00", "completed": true }
```
응답 `200` 수정된 plan 객체.
> 완료 체크/해제는 `{ "completed": true }` 또는 `{ "completed": false }`만 보내면 됩니다.

#### DELETE `/api/plans/{id}` — 삭제
응답 `200`
```json
{ "message": "삭제 완료" }
```

---

### 사진

#### POST `/api/plans/{id}/photos` — 업로드 (인증 필요, multipart)
- `multipart/form-data`, 필드명 **`photo`** (여러 개 가능).
- 허용 형식: jpeg / png / webp / gif.
- **주의:** fetch로 `FormData`를 보낼 때 `Content-Type` 헤더를 직접 지정하지 마세요. 브라우저가 boundary를 자동으로 설정합니다.
- 응답 `200` — `photos` 배열이 갱신된 plan 객체.

#### GET `/api/photos/{filename}` — 이미지 서빙 (인증 불필요)
- `plan.photos`의 각 파일명을 그대로 사용.
- 이미지 표시: `<img src="http://localhost:8080/api/photos/a1b2.jpg" />`

---

## 4. React 연동 예시

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

## 5. 프론트팀 체크리스트 / 주의사항

- 인증 필요한 모든 요청에 `Authorization: Bearer <token>` 빠뜨리지 않기. (누락 시 401)
- 사진 업로드 시 `Content-Type` 수동 설정 금지 (FormData 자동 처리).
- 날짜는 반드시 `"YYYY-MM-DD"` 문자열로 전송. `Date` 객체 그대로 보내지 말 것.
- `plan.id`는 경로에 그대로 사용 (접두사 없음). `user.id`는 `"user:..."` 형태이지만 프론트에서 경로로 쓸 일은 없음.
- 에러는 모두 `{ "error": "..." }` 형태 → 위 래퍼처럼 `data.error`로 메시지 추출.
- 토큰 만료(7일) 또는 무효 시 401 → 로그인 화면으로 유도.

## 6. 백엔드와 합의가 더 필요한 항목 (참고)
- **배포 시 CORS**: 현재 전체 허용. 운영 환경에서는 프론트 도메인만 허용하도록 좁힐 예정 → 프론트 배포 URL 공유 필요.
- **이미지 인증**: `GET /api/photos/{filename}`은 현재 인증 없이 누구나 접근 가능. 비공개가 필요하면 추후 토큰 검증 추가 예정(이 경우 `<img src>` 직접 사용 대신 별도 처리 필요).
- **페이지네이션**: 플랜 목록은 현재 전량 반환. 데이터가 많아지면 페이징 추가 가능.
