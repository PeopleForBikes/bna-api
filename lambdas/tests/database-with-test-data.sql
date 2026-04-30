--
-- PostgreSQL database dump
--

\restrict ExtkqABOWrvx0DUx9HgDijXj8f8FIkZV0M1JJ2wUGeXfQTiEqA2MdbWllxtsmjZ

-- Dumped from database version 17.9 (Debian 17.9-1.pgdg13+1)
-- Dumped by pg_dump version 17.9 (Homebrew)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: approval_status; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.approval_status (
    status character varying NOT NULL
);


ALTER TABLE public.approval_status OWNER TO postgres;

--
-- Name: bike_lane_type; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.bike_lane_type (
    name character varying NOT NULL
);


ALTER TABLE public.bike_lane_type OWNER TO postgres;

--
-- Name: bna_pipeline; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.bna_pipeline (
    state_machine_id uuid NOT NULL,
    step character varying NOT NULL,
    sqs_message json,
    fargate_price_id integer,
    fargate_task_arn character varying,
    s3_bucket character varying,
    status character varying DEFAULT 'Pending'::character varying NOT NULL,
    start_time timestamp with time zone NOT NULL,
    end_time timestamp with time zone,
    cost numeric
);


ALTER TABLE public.bna_pipeline OWNER TO postgres;

--
-- Name: bna_pipeline_status; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.bna_pipeline_status (
    status character varying NOT NULL
);


ALTER TABLE public.bna_pipeline_status OWNER TO postgres;

--
-- Name: bna_pipeline_step; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.bna_pipeline_step (
    step character varying NOT NULL
);


ALTER TABLE public.bna_pipeline_step OWNER TO postgres;

--
-- Name: bna_region; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.bna_region (
    name character varying NOT NULL
);


ALTER TABLE public.bna_region OWNER TO postgres;

--
-- Name: city; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.city (
    id uuid NOT NULL,
    country character varying NOT NULL,
    state character varying NOT NULL,
    name character varying NOT NULL,
    latitude double precision,
    longitude double precision,
    region character varying,
    state_abbrev character varying,
    residential_speed_limit integer,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone,
    fips_code character varying
);


ALTER TABLE public.city OWNER TO postgres;

--
-- Name: core_services; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.core_services (
    id uuid NOT NULL,
    dentists double precision,
    doctors double precision,
    grocery double precision,
    hospitals double precision,
    pharmacies double precision,
    score double precision,
    social_services double precision
);


ALTER TABLE public.core_services OWNER TO postgres;

--
-- Name: country; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.country (
    name character varying NOT NULL
);


ALTER TABLE public.country OWNER TO postgres;

--
-- Name: fargate_price; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.fargate_price (
    id integer NOT NULL,
    per_second numeric NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.fargate_price OWNER TO postgres;

--
-- Name: fargate_price_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.fargate_price_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.fargate_price_id_seq OWNER TO postgres;

--
-- Name: fargate_price_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.fargate_price_id_seq OWNED BY public.fargate_price.id;


--
-- Name: infrastructure; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.infrastructure (
    id uuid NOT NULL,
    low_stress_miles double precision,
    high_stress_miles double precision
);


ALTER TABLE public.infrastructure OWNER TO postgres;

--
-- Name: measure; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.measure (
    id uuid NOT NULL,
    buffered_lane double precision,
    lane double precision,
    path double precision,
    sharrow double precision,
    track double precision
);


ALTER TABLE public.measure OWNER TO postgres;

--
-- Name: opportunity; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.opportunity (
    id uuid NOT NULL,
    employment double precision,
    higher_education double precision,
    k12_education double precision,
    score double precision,
    technical_vocational_college double precision
);


ALTER TABLE public.opportunity OWNER TO postgres;

--
-- Name: people; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.people (
    id uuid NOT NULL,
    score double precision
);


ALTER TABLE public.people OWNER TO postgres;

--
-- Name: recreation; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.recreation (
    id uuid NOT NULL,
    community_centers double precision,
    parks double precision,
    recreation_trails double precision,
    score double precision
);


ALTER TABLE public.recreation OWNER TO postgres;

--
-- Name: retail; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.retail (
    id uuid NOT NULL,
    score double precision
);


ALTER TABLE public.retail OWNER TO postgres;

--
-- Name: seaql_migrations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.seaql_migrations (
    version character varying NOT NULL,
    applied_at bigint NOT NULL
);


ALTER TABLE public.seaql_migrations OWNER TO postgres;

--
-- Name: state_region_crosswalk; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.state_region_crosswalk (
    state character varying NOT NULL,
    region character varying NOT NULL
);


ALTER TABLE public.state_region_crosswalk OWNER TO postgres;

--
-- Name: submission; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.submission (
    id integer NOT NULL,
    first_name character varying NOT NULL,
    last_name character varying NOT NULL,
    occupation character varying,
    organization character varying,
    email character varying NOT NULL,
    country character varying NOT NULL,
    city character varying NOT NULL,
    region character varying,
    fips_code character varying DEFAULT '0'::character varying NOT NULL,
    consent boolean NOT NULL,
    status character varying NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.submission OWNER TO postgres;

--
-- Name: submission_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.submission_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.submission_id_seq OWNER TO postgres;

--
-- Name: submission_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.submission_id_seq OWNED BY public.submission.id;


--
-- Name: summary; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.summary (
    id uuid NOT NULL,
    city_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    pop_size integer NOT NULL,
    population integer NOT NULL,
    residential_speed_limit_override integer,
    score double precision NOT NULL,
    version character varying NOT NULL
);


ALTER TABLE public.summary OWNER TO postgres;

--
-- Name: transit; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.transit (
    id uuid NOT NULL,
    score double precision
);


ALTER TABLE public.transit OWNER TO postgres;

--
-- Name: us_state; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.us_state (
    name character varying NOT NULL,
    abbrev character varying NOT NULL,
    fips_code character varying NOT NULL,
    speed_limit integer NOT NULL
);


ALTER TABLE public.us_state OWNER TO postgres;

--
-- Name: fargate_price id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.fargate_price ALTER COLUMN id SET DEFAULT nextval('public.fargate_price_id_seq'::regclass);


--
-- Name: submission id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.submission ALTER COLUMN id SET DEFAULT nextval('public.submission_id_seq'::regclass);


--
-- Data for Name: approval_status; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.approval_status (status) FROM stdin;
Pending
Approved
Rejected
\.


--
-- Data for Name: bike_lane_type; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.bike_lane_type (name) FROM stdin;
buffered_lane
lane
path
sharrow
track
\.


--
-- Data for Name: bna_pipeline; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.bna_pipeline (state_machine_id, step, sqs_message, fargate_price_id, fargate_task_arn, s3_bucket, status, start_time, end_time, cost) FROM stdin;
\.


--
-- Data for Name: bna_pipeline_status; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.bna_pipeline_status (status) FROM stdin;
Completed
Pending
Processing
\.


--
-- Data for Name: bna_pipeline_step; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.bna_pipeline_step (step) FROM stdin;
Analysis
Cleanup
Save
Setup
\.


--
-- Data for Name: bna_region; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.bna_region (name) FROM stdin;
Mid-Atlantic
Midwest
Mountain
New England
Pacific
South
\.


--
-- Data for Name: city; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.city (id, country, state, name, latitude, longitude, region, state_abbrev, residential_speed_limit, created_at, updated_at, fips_code) FROM stdin;
4ccfcf6b-17c9-4006-b1cd-7e2829d85d54	United States	Tennessee	Chattanooga	35.065958	-85.248386	South	TN	25	2025-03-21 17:25:30.378391+00	\N	4714000
5180283b-f201-49af-9e7a-a6fc243ef2df	United States	Oklahoma	Bartlesville	36.7376675	-95.948454	South	OK	\N	2025-03-22 03:04:35.389279+00	\N	4004450
d755b6e8-6dee-4cae-9ce8-3662be60f776	United States	California	Calimesa	33.9873574	-117.0542141	Pacific	CA	\N	2025-03-13 18:52:46.943757+00	\N	609864
a18348b9-fcbd-46de-975c-ee74119519c3	United States	California	Temecula	33.4930728	-117.1317341	Pacific	CA	25	2025-03-13 18:52:43.796763+00	\N	678120
bf5f3e41-f394-4c0b-9550-060802adc6fa	United States	Pennsylvania	Somerset	40.0050299	-79.0778554	Mid-Atlantic	PA	\N	2025-03-22 00:58:08.03203+00	\N	4271776
296149fe-5d0e-4ee4-ab8b-e1e548ad0bf2	United States	Missouri	Columbia	38.9472896	-92.3263872	Midwest	MO	\N	2025-03-24 23:15:26.968664+00	\N	2915670
7a645a59-1477-4fc0-83ad-75f55129b09f	United States	Georgia	Roberta	32.7197762	-84.0103813	South	GA	\N	2025-03-23 20:45:26.24385+00	\N	1365856
419779dc-65f1-410a-a55d-42c379e252de	United States	Oklahoma	Muskogee	35.7437972	-95.3564048	South	OK	\N	2025-03-22 03:04:35.086807+00	\N	4050050
df866685-6784-4f95-bcab-8c9438c57630	United States	Colorado	Central City	39.7953658	-105.5089645	Mountain	CO	\N	2025-03-15 13:52:43.092106+00	\N	812900
3aea277c-9c05-44e2-87a5-6a1b900bf0d2	Canada	Ontario	Oshawa	43.8971	-78.8658	Canada	ON	\N	2025-04-21 03:40:23.247091+00	\N	0
8fbe8758-1dc7-4da1-8926-34a6baf5858a	United States	Hawaii	Waipahu	21.3849378	-158.0109928	Pacific	HI	\N	2025-03-26 18:51:33.67813+00	\N	1579700
\.


--
-- Data for Name: core_services; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.core_services (id, dentists, doctors, grocery, hospitals, pharmacies, score, social_services) FROM stdin;
bcbbe298-9cef-42e9-81bd-4e026b72f351	40.69	8.81	27.61	37.99	11.3	23.81	15.66
f838f8c1-9833-45c9-acdf-f55218c7c13f	39.9	18.63	25.34	38.42	6.08	24.75	16.02
0b8fc358-3e4d-44fd-8368-59f5cc9a7007	40.33	21.12	28.75	43.41	13.98	28.01	16.59
fb12b2d7-dfaa-46f3-b63c-4c7b05e7824e	40.29	21.15	28.57	43.8	14.03	28.06	16.61
f0702ab1-48f9-45dc-861e-3aed6c20b310	40.82	21.73	29.22	44.26	13.19	27.76	12.37
27795032-4c19-4b73-9466-87b1573a6212	23.07	22.45	28.66	43.8	13.16	25.89	12.37
e32e5c1e-72ce-4d1b-9a32-7467ac7448aa	23.07	21.11	27.34	34.6	11.54	23.29	12.37
24882ecf-dfbf-4eef-a36c-eec23648a8c2	25.42	21.11	32.29	34.6	12.29	24.81	12.15
14b557ed-19fb-4bf6-bab4-37a02ebf0b61	25.42	21.34	32.49	34.68	14.66	25.16	12.15
666bddf7-5e3b-4def-ad36-960f805877cb	29.91	30.19	35.6	36.95	20.2	28.96	10.83
7eaf136b-9a57-435d-8591-483de5610b5a	11.57	16.36	21.3	12.23	15.08	16.13	0
2519af81-b920-4707-bed6-8e56d93bcc74	11.57	16.36	21.3	12.23	15.08	16.13	0
9b43f089-2932-45da-b924-613e643ca7f2	11.57	15.48	21.3	12.23	15.08	15.92	0
bb290e88-d405-45d8-b604-6eb5e3a0e65c	11.57	15.48	21.3	12.23	15.08	15.92	0
fb09d896-e363-4be0-9c4c-255acd0e2279	12.42	13.78	21.79	12.23	15.15	17.39	26.54
42cf1436-f053-44bd-9859-550ea313ec95	12.42	13.78	21.79	12.23	15.15	17.39	26.54
ae7bcb6d-69cc-4ece-9acb-c64730079869	12.42	13.59	20.5	12.23	13.94	16.91	26.54
a79deacf-525f-4b3c-800b-0dd9b8d66b33	12.42	13.15	18.69	12.23	15.63	15.68	20.81
96a55ca7-2c80-42ad-acd3-92e7917ffd6d	15.57	16.74	20.46	2.75	15.6	14.92	18.59
5ed0f710-edce-4b32-8512-acee72d6956e	0	0	4.96	10.22	0	7.3	0
01674cb0-7f28-41b8-be65-f9ad36bb9272	0	0	18.98	10.22	0	15.09	0
380a7ec6-d1c8-4723-b252-cfa5d06a33e3	0	0	19.19	10.22	46.26	20.85	0
a9282493-85d7-4262-8a20-10eec8be2576	2.75	7.52	9.45	7.47	0	7.51	0
8569094d-b182-40a2-995a-8219c4a8fcfc	2.75	7.52	9.45	7.47	0	7.51	0
48ee9912-28fe-4582-8c4d-23c2ec9e19ac	2.75	7.52	9.45	7.47	0	7.51	0
66e766ad-82aa-4b1a-865f-9af40886866a	2.75	7.52	9.45	7.47	0	7.51	0
201e3ed8-cdc8-4d0a-ac48-43f34bba6157	2.75	7.52	9.45	7.47	0	7.51	0
c222a999-3b81-43f4-b8ff-1edf62492c02	2.75	7.52	9.45	7.47	4.87	7.2	0
b33f8354-dfce-4a16-ab27-22297226e01f	2.75	5.2	9.45	7.47	2.77	6.41	0
41430704-c565-47a3-b97a-0b9950be7d87	2.63	0	9.26	0	1.25	6.01	0
856be1f6-eb45-45ea-b1bf-ffb5dd0643e0	17.12	19.65	40.62	8.21	24.86	24.21	28.58
cd2909ff-81c9-4006-bdda-579a233c3911	17.63	19.7	43.03	11.06	31.01	26.45	31.15
aa938808-37fc-429c-b79f-b6295b1f9917	17.36	19.7	45.68	11.06	30.84	27.12	31.55
87b533b6-b5b0-4a7b-a8eb-93de2a8ca054	26	28.23	62.22	11.96	39.98	36.38	41.25
1eeab18e-a24c-4321-8f3f-14f52f9360be	38.3	33.94	68.08	17.44	47.46	43.26	49.24
c95dfd2d-f989-4c10-a9d6-c8450a488646	21.56	10.02	25.69	18.74	23.02	21.79	34.36
6b470afd-0f87-4bdd-a9b2-1e3be794a053	21.56	10.02	25.69	18.74	23.02	21.79	34.36
aede142b-f3d3-45a6-bf7c-66c05b33f5a3	21.86	10.9	26.35	18.74	16.25	21.48	34.36
cf05a5f2-55f9-4d32-a011-585b4c76b5cd	22.23	13.69	33.18	18.74	17.77	23.94	34.36
7e354518-6652-4685-a9ab-bf385f184683	0	0	21.36	3.04	9.95	10.64	10.44
d106416f-849b-4b60-812c-a9633faa1b91	0.67	0	21.36	2.13	9.95	10.49	10.44
5b6a2e81-ed02-4052-9428-b392c09d153d	0.67	0	21.36	2.13	9.95	10.68	11.45
8685dc55-b4aa-4977-b9ef-268b5deb872e	0.67	0	26.07	2.13	9.95	12.15	11.45
ee2c925a-a9cd-4bde-8beb-9b3868c5d4b8	0.67	0	26.07	2.13	9.95	9.21	8.02
a6c23082-a0fc-460b-87a7-bf2d6a8fa6d7	8.85	0	36.44	2.09	17.2	13.4	8.46
f2b65b6c-fb24-4b11-a44c-5d8c7d4c5b69	0	0	0	0	0	0	0
1dce7b76-ea70-47eb-96da-7a88f61d4470	0	0	0	0	0	0	0
52d83e7d-14b6-4352-bcaf-f3c673d61c93	0	0	0	0	0	0	0
002f5f3d-e6fe-4765-a7d3-1d07fec21539	0	0	0	0	0	0	0
65b30778-8c21-41ff-aa76-5e4f8030d12d	0	0	0	0	0	0	0
7e00bd6d-eaa5-4e7a-95fe-3c8f03427b3e	0	0	0	0	0	0	0
51937226-98db-4d5d-808b-6960c27d6eb3	0	0	0	0	0	0	0
7d4e53e1-85de-45fa-b417-0ce83aa61828	0	0	0	0	0	0	0
a8d4651d-42bc-40e0-8310-6c6e3fa70431	0	0	0	0	0	\N	0
b9cd336e-57eb-46f8-b3bf-e2eb3a5ebd43	0	0	0	0	0	\N	0
8f58f058-7bd5-4e1d-97ee-377890e10eba	21.63	15.76	26.5	14.19	12.55	18.86	0
4fe2fcac-f557-4505-9b42-7c01d6b30ffd	22.41	10.69	26	14.44	13	17.73	0
49cdb0fe-00b3-4ca1-b0d9-31e93c202185	22.91	10.69	27.18	13.39	13.25	17.91	0
a2cb3e9f-4a8a-4ac8-b12a-53bd5c792fbc	20.36	9.25	25.72	14.38	12.72	16.64	14.49
eb38ad18-ffbf-45eb-bf01-931deb98c1dc	17.74	8.75	23.52	13.89	19.32	15.93	12.13
d7c5d8df-4b3c-4734-84e5-c364fd626cf3	16.59	9.21	26.54	11.8	16	15.5	9.34
7b138125-e81c-4b00-b1ad-11ed620685cd	17.07	7	25.28	11.81	18.33	15.19	10.43
c1235fcf-82c6-4cf9-bd18-4a457b4f0a13	15.03	12.47	20.27	14.5	15.96	15.07	10.03
0724c17a-750b-4db4-af45-68b8103eec77	14.69	12.81	19.85	14.33	15.93	14.96	10.03
8fd7642f-4a31-4a13-95c8-f1ea1aa263ba	14.69	12.54	20.55	14.63	15.93	15.1	9.75
e09fb7d5-dd65-44db-a963-d07523215d84	14.69	13.49	20.55	14.65	19.64	15.66	9.75
5885ec0d-6af5-43c1-9978-d2389e40b95f	13.45	17.91	23.83	16.44	20.53	19.36	20.91
\.


--
-- Data for Name: country; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.country (name) FROM stdin;
Australia
Belgium
Brazil
Canada
Chile
Colombia
Croatia
Cuba
England
France
Germany
Greece
Guatemala
Iran
Iraq
Ireland
Italy
Mexico
Netherlands
New Zealand
Northern Ireland
Portugal
Scotland
Spain
United States
Vietnam
Wales
\.


--
-- Data for Name: fargate_price; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.fargate_price (id, per_second, created_at) FROM stdin;
1	0.000038	2026-04-30 14:55:47.582288+00
\.


--
-- Data for Name: infrastructure; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.infrastructure (id, low_stress_miles, high_stress_miles) FROM stdin;
bcbbe298-9cef-42e9-81bd-4e026b72f351	1051.4	362.8
f838f8c1-9833-45c9-acdf-f55218c7c13f	1054.3	368.8
0b8fc358-3e4d-44fd-8368-59f5cc9a7007	1081	365
fb12b2d7-dfaa-46f3-b63c-4c7b05e7824e	1092.9	364.6
f0702ab1-48f9-45dc-861e-3aed6c20b310	1104.8	366.1
27795032-4c19-4b73-9466-87b1573a6212	1119.1	374
e32e5c1e-72ce-4d1b-9a32-7467ac7448aa	1119.8	374.3
24882ecf-dfbf-4eef-a36c-eec23648a8c2	1118.9	372.1
14b557ed-19fb-4bf6-bab4-37a02ebf0b61	1118.4	373.5
666bddf7-5e3b-4def-ad36-960f805877cb	1147.1	389.7
7eaf136b-9a57-435d-8591-483de5610b5a	562.3	256.2
2519af81-b920-4707-bed6-8e56d93bcc74	576.7	255.9
9b43f089-2932-45da-b924-613e643ca7f2	576.7	255.9
bb290e88-d405-45d8-b604-6eb5e3a0e65c	577.2	256.8
fb09d896-e363-4be0-9c4c-255acd0e2279	594.4	249.7
42cf1436-f053-44bd-9859-550ea313ec95	597.8	249.8
ae7bcb6d-69cc-4ece-9acb-c64730079869	592	250.7
a79deacf-525f-4b3c-800b-0dd9b8d66b33	594.9	250.6
96a55ca7-2c80-42ad-acd3-92e7917ffd6d	606.4	250.7
5ed0f710-edce-4b32-8512-acee72d6956e	68.7	16.2
01674cb0-7f28-41b8-be65-f9ad36bb9272	68.7	16.2
380a7ec6-d1c8-4723-b252-cfa5d06a33e3	68.7	16.2
a9282493-85d7-4262-8a20-10eec8be2576	420.5	111.3
8569094d-b182-40a2-995a-8219c4a8fcfc	420.7	111.3
48ee9912-28fe-4582-8c4d-23c2ec9e19ac	424.1	111.1
66e766ad-82aa-4b1a-865f-9af40886866a	424.3	111.1
201e3ed8-cdc8-4d0a-ac48-43f34bba6157	424.3	111.1
c222a999-3b81-43f4-b8ff-1edf62492c02	423.5	111.1
b33f8354-dfce-4a16-ab27-22297226e01f	413.8	122.2
41430704-c565-47a3-b97a-0b9950be7d87	125.4	38.7
856be1f6-eb45-45ea-b1bf-ffb5dd0643e0	174.9	862.4
cd2909ff-81c9-4006-bdda-579a233c3911	197.5	861.3
aa938808-37fc-429c-b79f-b6295b1f9917	205.3	863.7
87b533b6-b5b0-4a7b-a8eb-93de2a8ca054	771.1	306.3
1eeab18e-a24c-4321-8f3f-14f52f9360be	825.2	313.8
c95dfd2d-f989-4c10-a9d6-c8450a488646	605.7	299.1
6b470afd-0f87-4bdd-a9b2-1e3be794a053	604.9	299.1
aede142b-f3d3-45a6-bf7c-66c05b33f5a3	607	295.9
cf05a5f2-55f9-4d32-a011-585b4c76b5cd	621.2	289.5
7e354518-6652-4685-a9ab-bf385f184683	70.4	33
d106416f-849b-4b60-812c-a9633faa1b91	70.5	33
5b6a2e81-ed02-4052-9428-b392c09d153d	70.5	33
8685dc55-b4aa-4977-b9ef-268b5deb872e	70.4	33
ee2c925a-a9cd-4bde-8beb-9b3868c5d4b8	70.4	33
a6c23082-a0fc-460b-87a7-bf2d6a8fa6d7	71.9	30.8
f2b65b6c-fb24-4b11-a44c-5d8c7d4c5b69	7.3	34.1
1dce7b76-ea70-47eb-96da-7a88f61d4470	7.3	34.1
52d83e7d-14b6-4352-bcaf-f3c673d61c93	7.3	34.1
002f5f3d-e6fe-4765-a7d3-1d07fec21539	7.3	34.1
65b30778-8c21-41ff-aa76-5e4f8030d12d	7.3	34.1
7e00bd6d-eaa5-4e7a-95fe-3c8f03427b3e	7.3	34.1
51937226-98db-4d5d-808b-6960c27d6eb3	0	23.9
7d4e53e1-85de-45fa-b417-0ce83aa61828	0	23.9
a8d4651d-42bc-40e0-8310-6c6e3fa70431	0	23.9
b9cd336e-57eb-46f8-b3bf-e2eb3a5ebd43	0.5	23.8
8f58f058-7bd5-4e1d-97ee-377890e10eba	0	0
4fe2fcac-f557-4505-9b42-7c01d6b30ffd	2135.8	604.1
49cdb0fe-00b3-4ca1-b0d9-31e93c202185	2132.3	604.3
a2cb3e9f-4a8a-4ac8-b12a-53bd5c792fbc	2137.8	606.8
eb38ad18-ffbf-45eb-bf01-931deb98c1dc	2178.4	613.9
d7c5d8df-4b3c-4734-84e5-c364fd626cf3	2175.5	620.3
7b138125-e81c-4b00-b1ad-11ed620685cd	2165.5	623.1
c1235fcf-82c6-4cf9-bd18-4a457b4f0a13	2159.4	682.7
0724c17a-750b-4db4-af45-68b8103eec77	2156.5	683.9
8fd7642f-4a31-4a13-95c8-f1ea1aa263ba	2150.4	683.4
e09fb7d5-dd65-44db-a963-d07523215d84	2153.5	682.4
5885ec0d-6af5-43c1-9978-d2389e40b95f	2175.5	682.8
\.


--
-- Data for Name: measure; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.measure (id, buffered_lane, lane, path, sharrow, track) FROM stdin;
\.


--
-- Data for Name: opportunity; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.opportunity (id, employment, higher_education, k12_education, score, technical_vocational_college) FROM stdin;
bcbbe298-9cef-42e9-81bd-4e026b72f351	26.52	39.15	28.21	31.13	41.4
f838f8c1-9833-45c9-acdf-f55218c7c13f	25.7	38.6	27.24	30.22	39.68
0b8fc358-3e4d-44fd-8368-59f5cc9a7007	28.48	40.59	28.97	32.23	40.02
fb12b2d7-dfaa-46f3-b63c-4c7b05e7824e	28.18	40.62	28.99	32.14	40.04
f0702ab1-48f9-45dc-861e-3aed6c20b310	27.69	40.97	29.04	32.11	40.64
27795032-4c19-4b73-9466-87b1573a6212	28.19	41.53	29.52	32.53	40.3
e32e5c1e-72ce-4d1b-9a32-7467ac7448aa	27.92	41.53	29.38	32.39	40.3
24882ecf-dfbf-4eef-a36c-eec23648a8c2	28.07	41.56	29.41	32.46	40.3
14b557ed-19fb-4bf6-bab4-37a02ebf0b61	28.21	41.81	29.48	32.63	40.8
666bddf7-5e3b-4def-ad36-960f805877cb	30.31	43.43	32.51	33.66	29.85
7eaf136b-9a57-435d-8591-483de5610b5a	18.2	13.89	38.6	23.81	11.53
2519af81-b920-4707-bed6-8e56d93bcc74	18.2	13.89	38.6	23.81	11.53
9b43f089-2932-45da-b924-613e643ca7f2	18.2	13.89	38.6	23.81	11.53
bb290e88-d405-45d8-b604-6eb5e3a0e65c	18.2	13.89	38.6	24.24	15.83
fb09d896-e363-4be0-9c4c-255acd0e2279	17.24	15.77	36.93	23.7	15.83
42cf1436-f053-44bd-9859-550ea313ec95	17.23	15.77	36.88	23.68	15.83
ae7bcb6d-69cc-4ece-9acb-c64730079869	16.43	15.77	35.94	23.07	15.83
a79deacf-525f-4b3c-800b-0dd9b8d66b33	15.97	15.77	38.78	23.9	15.83
96a55ca7-2c80-42ad-acd3-92e7917ffd6d	16.32	15.77	40.1	24.48	15.83
5ed0f710-edce-4b32-8512-acee72d6956e	37.1	0	54.77	45.94	0
01674cb0-7f28-41b8-be65-f9ad36bb9272	37.1	0	54.77	45.94	0
380a7ec6-d1c8-4723-b252-cfa5d06a33e3	37.65	0	54.77	46.21	0
a9282493-85d7-4262-8a20-10eec8be2576	21.52	0	36.87	27.33	14.3
8569094d-b182-40a2-995a-8219c4a8fcfc	21.52	0	36.87	27.33	14.3
48ee9912-28fe-4582-8c4d-23c2ec9e19ac	22.93	0	37.01	28.01	14.32
66e766ad-82aa-4b1a-865f-9af40886866a	22.92	0	37.01	28.01	14.32
201e3ed8-cdc8-4d0a-ac48-43f34bba6157	22.92	0	35.51	27.31	13.99
c222a999-3b81-43f4-b8ff-1edf62492c02	22.9	0	35.5	27.3	13.99
b33f8354-dfce-4a16-ab27-22297226e01f	26.46	0	33.82	28.12	13.99
41430704-c565-47a3-b97a-0b9950be7d87	32.8	0	33.29	33.05	0
856be1f6-eb45-45ea-b1bf-ffb5dd0643e0	0	15.24	50.31	34.9	17.69
cd2909ff-81c9-4006-bdda-579a233c3911	0	23.84	51.04	39.56	28.88
aa938808-37fc-429c-b79f-b6295b1f9917	0	23.84	51.41	39.46	28.88
87b533b6-b5b0-4a7b-a8eb-93de2a8ca054	0	26.38	61.98	45.84	28.29
1eeab18e-a24c-4321-8f3f-14f52f9360be	0	27.69	65.94	48.89	31.61
c95dfd2d-f989-4c10-a9d6-c8450a488646	22.27	2.31	31.07	21.23	20.98
6b470afd-0f87-4bdd-a9b2-1e3be794a053	22.34	2.31	31.07	21.25	20.98
aede142b-f3d3-45a6-bf7c-66c05b33f5a3	22.82	2.31	31.28	21.5	20.98
cf05a5f2-55f9-4d32-a011-585b4c76b5cd	24.08	2.31	31.24	21.66	18.32
7e354518-6652-4685-a9ab-bf385f184683	13.4	0	17.15	14.12	6.04
d106416f-849b-4b60-812c-a9633faa1b91	13.59	0	17.15	14.2	6.04
5b6a2e81-ed02-4052-9428-b392c09d153d	13.66	0	17.57	14.42	6.04
8685dc55-b4aa-4977-b9ef-268b5deb872e	14.34	0	18.59	15.16	6.04
ee2c925a-a9cd-4bde-8beb-9b3868c5d4b8	14.34	0	18.59	15.16	6.04
a6c23082-a0fc-460b-87a7-bf2d6a8fa6d7	15.43	0	27.38	19.59	6.92
f2b65b6c-fb24-4b11-a44c-5d8c7d4c5b69	7.21	0	0	3.61	0
1dce7b76-ea70-47eb-96da-7a88f61d4470	7.21	0	0	3.61	0
52d83e7d-14b6-4352-bcaf-f3c673d61c93	7.21	0	0	3.61	0
002f5f3d-e6fe-4765-a7d3-1d07fec21539	7.21	0	0	3.61	0
65b30778-8c21-41ff-aa76-5e4f8030d12d	7.21	0	0	3.61	0
7e00bd6d-eaa5-4e7a-95fe-3c8f03427b3e	7.21	0	0	3.61	0
51937226-98db-4d5d-808b-6960c27d6eb3	28.04	0	14.46	21.25	0
7d4e53e1-85de-45fa-b417-0ce83aa61828	28.04	0	14.46	21.25	0
a8d4651d-42bc-40e0-8310-6c6e3fa70431	28.04	0	14.46	21.25	0
b9cd336e-57eb-46f8-b3bf-e2eb3a5ebd43	28.04	0	14.46	21.25	0
8f58f058-7bd5-4e1d-97ee-377890e10eba	24.01	16.88	36.11	24.93	5.14
4fe2fcac-f557-4505-9b42-7c01d6b30ffd	24.12	10.28	35.8	23.54	5.14
49cdb0fe-00b3-4ca1-b0d9-31e93c202185	24.09	10.28	35.95	23.58	5.14
a2cb3e9f-4a8a-4ac8-b12a-53bd5c792fbc	24.19	11.75	36.14	23.98	5.14
eb38ad18-ffbf-45eb-bf01-931deb98c1dc	24.75	10.96	32.87	22.81	4.47
d7c5d8df-4b3c-4734-84e5-c364fd626cf3	24.44	12.48	33.57	23.25	4.47
7b138125-e81c-4b00-b1ad-11ed620685cd	24.95	12.5	33.84	23.64	5.63
c1235fcf-82c6-4cf9-bd18-4a457b4f0a13	21.52	21	29.86	22.71	5.25
0724c17a-750b-4db4-af45-68b8103eec77	21.28	20.78	29.32	22.39	5.25
8fd7642f-4a31-4a13-95c8-f1ea1aa263ba	21.19	19.66	29.14	22.07	5.25
e09fb7d5-dd65-44db-a963-d07523215d84	21.24	19.66	29.92	22.36	5.25
5885ec0d-6af5-43c1-9978-d2389e40b95f	22.64	20.22	31.35	23.51	5.73
\.


--
-- Data for Name: people; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.people (id, score) FROM stdin;
bcbbe298-9cef-42e9-81bd-4e026b72f351	38.45
f838f8c1-9833-45c9-acdf-f55218c7c13f	36.79
0b8fc358-3e4d-44fd-8368-59f5cc9a7007	39.38
fb12b2d7-dfaa-46f3-b63c-4c7b05e7824e	39.39
f0702ab1-48f9-45dc-861e-3aed6c20b310	39.44
27795032-4c19-4b73-9466-87b1573a6212	39.35
e32e5c1e-72ce-4d1b-9a32-7467ac7448aa	38.86
24882ecf-dfbf-4eef-a36c-eec23648a8c2	38.72
14b557ed-19fb-4bf6-bab4-37a02ebf0b61	38.79
666bddf7-5e3b-4def-ad36-960f805877cb	39.3
7eaf136b-9a57-435d-8591-483de5610b5a	26.18
2519af81-b920-4707-bed6-8e56d93bcc74	26.24
9b43f089-2932-45da-b924-613e643ca7f2	26.24
bb290e88-d405-45d8-b604-6eb5e3a0e65c	26.24
fb09d896-e363-4be0-9c4c-255acd0e2279	26.65
42cf1436-f053-44bd-9859-550ea313ec95	26.62
ae7bcb6d-69cc-4ece-9acb-c64730079869	24.85
a79deacf-525f-4b3c-800b-0dd9b8d66b33	23.99
96a55ca7-2c80-42ad-acd3-92e7917ffd6d	24.16
5ed0f710-edce-4b32-8512-acee72d6956e	47.04
01674cb0-7f28-41b8-be65-f9ad36bb9272	47.04
380a7ec6-d1c8-4723-b252-cfa5d06a33e3	47.14
a9282493-85d7-4262-8a20-10eec8be2576	41.88
8569094d-b182-40a2-995a-8219c4a8fcfc	41.88
48ee9912-28fe-4582-8c4d-23c2ec9e19ac	42.13
66e766ad-82aa-4b1a-865f-9af40886866a	42.13
201e3ed8-cdc8-4d0a-ac48-43f34bba6157	42.13
c222a999-3b81-43f4-b8ff-1edf62492c02	42.13
b33f8354-dfce-4a16-ab27-22297226e01f	40.67
41430704-c565-47a3-b97a-0b9950be7d87	42.71
856be1f6-eb45-45ea-b1bf-ffb5dd0643e0	23.25
cd2909ff-81c9-4006-bdda-579a233c3911	27.29
aa938808-37fc-429c-b79f-b6295b1f9917	27.27
87b533b6-b5b0-4a7b-a8eb-93de2a8ca054	36.5
1eeab18e-a24c-4321-8f3f-14f52f9360be	40.95
c95dfd2d-f989-4c10-a9d6-c8450a488646	35.23
6b470afd-0f87-4bdd-a9b2-1e3be794a053	35.23
aede142b-f3d3-45a6-bf7c-66c05b33f5a3	35.47
cf05a5f2-55f9-4d32-a011-585b4c76b5cd	36.96
7e354518-6652-4685-a9ab-bf385f184683	14.92
d106416f-849b-4b60-812c-a9633faa1b91	14.92
5b6a2e81-ed02-4052-9428-b392c09d153d	15.14
8685dc55-b4aa-4977-b9ef-268b5deb872e	15.63
ee2c925a-a9cd-4bde-8beb-9b3868c5d4b8	15.63
a6c23082-a0fc-460b-87a7-bf2d6a8fa6d7	17.35
f2b65b6c-fb24-4b11-a44c-5d8c7d4c5b69	35.61
1dce7b76-ea70-47eb-96da-7a88f61d4470	35.61
52d83e7d-14b6-4352-bcaf-f3c673d61c93	35.61
002f5f3d-e6fe-4765-a7d3-1d07fec21539	35.61
65b30778-8c21-41ff-aa76-5e4f8030d12d	35.61
7e00bd6d-eaa5-4e7a-95fe-3c8f03427b3e	35.61
51937226-98db-4d5d-808b-6960c27d6eb3	31
7d4e53e1-85de-45fa-b417-0ce83aa61828	31
a8d4651d-42bc-40e0-8310-6c6e3fa70431	31
b9cd336e-57eb-46f8-b3bf-e2eb3a5ebd43	31.16
8f58f058-7bd5-4e1d-97ee-377890e10eba	37.29
4fe2fcac-f557-4505-9b42-7c01d6b30ffd	36.83
49cdb0fe-00b3-4ca1-b0d9-31e93c202185	36.88
a2cb3e9f-4a8a-4ac8-b12a-53bd5c792fbc	37.1
eb38ad18-ffbf-45eb-bf01-931deb98c1dc	37.12
d7c5d8df-4b3c-4734-84e5-c364fd626cf3	37.01
7b138125-e81c-4b00-b1ad-11ed620685cd	37.43
c1235fcf-82c6-4cf9-bd18-4a457b4f0a13	32.2
0724c17a-750b-4db4-af45-68b8103eec77	31.87
8fd7642f-4a31-4a13-95c8-f1ea1aa263ba	31.71
e09fb7d5-dd65-44db-a963-d07523215d84	31.72
5885ec0d-6af5-43c1-9978-d2389e40b95f	32.49
\.


--
-- Data for Name: recreation; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.recreation (id, community_centers, parks, recreation_trails, score) FROM stdin;
bcbbe298-9cef-42e9-81bd-4e026b72f351	0	46.3	24.72	36.23
f838f8c1-9833-45c9-acdf-f55218c7c13f	0	46.2	23.11	35.42
0b8fc358-3e4d-44fd-8368-59f5cc9a7007	0	49.37	47.65	48.57
fb12b2d7-dfaa-46f3-b63c-4c7b05e7824e	0	50.04	47.62	48.91
f0702ab1-48f9-45dc-861e-3aed6c20b310	0	52.03	47.65	49.99
27795032-4c19-4b73-9466-87b1573a6212	0	52.87	47.47	50.35
e32e5c1e-72ce-4d1b-9a32-7467ac7448aa	0	53.1	47.47	50.47
24882ecf-dfbf-4eef-a36c-eec23648a8c2	0	53.51	47.51	50.71
14b557ed-19fb-4bf6-bab4-37a02ebf0b61	0	53.55	47.51	50.73
666bddf7-5e3b-4def-ad36-960f805877cb	29.58	57.8	56.85	50.41
7eaf136b-9a57-435d-8591-483de5610b5a	21.25	65.61	31.5	42.58
2519af81-b920-4707-bed6-8e56d93bcc74	21.28	65.58	31.5	42.58
9b43f089-2932-45da-b924-613e643ca7f2	21.28	65.58	31.5	42.58
bb290e88-d405-45d8-b604-6eb5e3a0e65c	21.28	65.62	31.5	42.59
fb09d896-e363-4be0-9c4c-255acd0e2279	25.36	65.88	32.83	44.18
42cf1436-f053-44bd-9859-550ea313ec95	25.19	65.95	32.83	44.17
ae7bcb6d-69cc-4ece-9acb-c64730079869	24.15	65.55	32.83	43.75
a79deacf-525f-4b3c-800b-0dd9b8d66b33	24.42	66.11	26.82	41.94
96a55ca7-2c80-42ad-acd3-92e7917ffd6d	24.5	66.34	26.86	42.06
5ed0f710-edce-4b32-8512-acee72d6956e	0	52.53	0	52.53
01674cb0-7f28-41b8-be65-f9ad36bb9272	0	52.53	0	52.53
380a7ec6-d1c8-4723-b252-cfa5d06a33e3	0	52.53	0	52.53
a9282493-85d7-4262-8a20-10eec8be2576	0	22.23	21.02	21.67
8569094d-b182-40a2-995a-8219c4a8fcfc	0	22.13	21.02	21.61
48ee9912-28fe-4582-8c4d-23c2ec9e19ac	0	25.36	13.78	19.96
66e766ad-82aa-4b1a-865f-9af40886866a	0	25.36	13.75	19.94
201e3ed8-cdc8-4d0a-ac48-43f34bba6157	0	26.07	13.75	20.32
c222a999-3b81-43f4-b8ff-1edf62492c02	0	26.09	13.75	20.33
b33f8354-dfce-4a16-ab27-22297226e01f	0	27.51	12.15	20.34
41430704-c565-47a3-b97a-0b9950be7d87	0	31.79	100	63.62
856be1f6-eb45-45ea-b1bf-ffb5dd0643e0	11.47	70.89	12.63	35.64
cd2909ff-81c9-4006-bdda-579a233c3911	16	71.87	18.36	39.17
aa938808-37fc-429c-b79f-b6295b1f9917	16.16	71.88	18.36	39.22
87b533b6-b5b0-4a7b-a8eb-93de2a8ca054	21.47	74.43	30.7	45.88
1eeab18e-a24c-4321-8f3f-14f52f9360be	27.85	75.73	27.4	46.84
c95dfd2d-f989-4c10-a9d6-c8450a488646	13.1	35.4	29.91	27.9
6b470afd-0f87-4bdd-a9b2-1e3be794a053	13.1	35.39	29.91	27.9
aede142b-f3d3-45a6-bf7c-66c05b33f5a3	13.72	35.92	31.75	28.91
cf05a5f2-55f9-4d32-a011-585b4c76b5cd	17.38	36.3	30.05	29.38
7e354518-6652-4685-a9ab-bf385f184683	5.61	16.71	0	12.44
d106416f-849b-4b60-812c-a9633faa1b91	5.61	16.26	7.39	10.49
5b6a2e81-ed02-4052-9428-b392c09d153d	6.02	18.53	7.45	11.52
8685dc55-b4aa-4977-b9ef-268b5deb872e	6.13	18.93	10.41	12.75
ee2c925a-a9cd-4bde-8beb-9b3868c5d4b8	6.13	18.93	10.41	12.75
a6c23082-a0fc-460b-87a7-bf2d6a8fa6d7	6.83	37.1	12.08	20.78
f2b65b6c-fb24-4b11-a44c-5d8c7d4c5b69	0	4.16	0	4.16
1dce7b76-ea70-47eb-96da-7a88f61d4470	0	4.16	0	4.16
52d83e7d-14b6-4352-bcaf-f3c673d61c93	0	4.16	0	4.16
002f5f3d-e6fe-4765-a7d3-1d07fec21539	0	4.16	0	4.16
65b30778-8c21-41ff-aa76-5e4f8030d12d	0	4.16	0	4.16
7e00bd6d-eaa5-4e7a-95fe-3c8f03427b3e	0	4.16	0	2.22
51937226-98db-4d5d-808b-6960c27d6eb3	0	2.05	0	2.05
7d4e53e1-85de-45fa-b417-0ce83aa61828	0	2.05	0	2.05
a8d4651d-42bc-40e0-8310-6c6e3fa70431	0	2.05	0	2.05
b9cd336e-57eb-46f8-b3bf-e2eb3a5ebd43	0	2.05	0	2.05
8f58f058-7bd5-4e1d-97ee-377890e10eba	0	28.95	18.9	24.26
4fe2fcac-f557-4505-9b42-7c01d6b30ffd	0	30.11	23.93	27.23
49cdb0fe-00b3-4ca1-b0d9-31e93c202185	6.78	31.27	20.84	21.5
a2cb3e9f-4a8a-4ac8-b12a-53bd5c792fbc	5.65	33.84	20.46	22.11
eb38ad18-ffbf-45eb-bf01-931deb98c1dc	7.08	40.66	19.98	25.03
d7c5d8df-4b3c-4734-84e5-c364fd626cf3	7.08	41.08	20.12	25.24
7b138125-e81c-4b00-b1ad-11ed620685cd	7.35	42.29	20.79	26.03
c1235fcf-82c6-4cf9-bd18-4a457b4f0a13	9.15	39.92	25.43	27.16
0724c17a-750b-4db4-af45-68b8103eec77	7.19	40.21	27.51	27.51
8fd7642f-4a31-4a13-95c8-f1ea1aa263ba	7.19	40.51	27.83	27.74
e09fb7d5-dd65-44db-a963-d07523215d84	7.19	40.6	24.86	26.74
5885ec0d-6af5-43c1-9978-d2389e40b95f	11.86	44.17	29.98	31.13
\.


--
-- Data for Name: retail; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.retail (id, score) FROM stdin;
bcbbe298-9cef-42e9-81bd-4e026b72f351	18.8
f838f8c1-9833-45c9-acdf-f55218c7c13f	22.96
0b8fc358-3e4d-44fd-8368-59f5cc9a7007	25.65
fb12b2d7-dfaa-46f3-b63c-4c7b05e7824e	25.76
f0702ab1-48f9-45dc-861e-3aed6c20b310	25.88
27795032-4c19-4b73-9466-87b1573a6212	25.98
e32e5c1e-72ce-4d1b-9a32-7467ac7448aa	25.22
24882ecf-dfbf-4eef-a36c-eec23648a8c2	25.96
14b557ed-19fb-4bf6-bab4-37a02ebf0b61	45.28
666bddf7-5e3b-4def-ad36-960f805877cb	47.54
7eaf136b-9a57-435d-8591-483de5610b5a	26.25
2519af81-b920-4707-bed6-8e56d93bcc74	26.25
9b43f089-2932-45da-b924-613e643ca7f2	26.25
bb290e88-d405-45d8-b604-6eb5e3a0e65c	26.25
fb09d896-e363-4be0-9c4c-255acd0e2279	26.62
42cf1436-f053-44bd-9859-550ea313ec95	26.54
ae7bcb6d-69cc-4ece-9acb-c64730079869	25.14
a79deacf-525f-4b3c-800b-0dd9b8d66b33	24.6
96a55ca7-2c80-42ad-acd3-92e7917ffd6d	25.49
5ed0f710-edce-4b32-8512-acee72d6956e	11.08
01674cb0-7f28-41b8-be65-f9ad36bb9272	13.07
380a7ec6-d1c8-4723-b252-cfa5d06a33e3	13.42
a9282493-85d7-4262-8a20-10eec8be2576	0
8569094d-b182-40a2-995a-8219c4a8fcfc	0
48ee9912-28fe-4582-8c4d-23c2ec9e19ac	0
66e766ad-82aa-4b1a-865f-9af40886866a	0
201e3ed8-cdc8-4d0a-ac48-43f34bba6157	16.5
c222a999-3b81-43f4-b8ff-1edf62492c02	20.25
b33f8354-dfce-4a16-ab27-22297226e01f	30.42
41430704-c565-47a3-b97a-0b9950be7d87	32.92
856be1f6-eb45-45ea-b1bf-ffb5dd0643e0	60.33
cd2909ff-81c9-4006-bdda-579a233c3911	62.74
aa938808-37fc-429c-b79f-b6295b1f9917	62.9
87b533b6-b5b0-4a7b-a8eb-93de2a8ca054	71.83
1eeab18e-a24c-4321-8f3f-14f52f9360be	74.05
c95dfd2d-f989-4c10-a9d6-c8450a488646	20.41
6b470afd-0f87-4bdd-a9b2-1e3be794a053	20.41
aede142b-f3d3-45a6-bf7c-66c05b33f5a3	34.52
cf05a5f2-55f9-4d32-a011-585b4c76b5cd	44.4
7e354518-6652-4685-a9ab-bf385f184683	0.67
d106416f-849b-4b60-812c-a9633faa1b91	0.67
5b6a2e81-ed02-4052-9428-b392c09d153d	0.67
8685dc55-b4aa-4977-b9ef-268b5deb872e	0.67
ee2c925a-a9cd-4bde-8beb-9b3868c5d4b8	6.22
a6c23082-a0fc-460b-87a7-bf2d6a8fa6d7	32.7
f2b65b6c-fb24-4b11-a44c-5d8c7d4c5b69	0
1dce7b76-ea70-47eb-96da-7a88f61d4470	0
52d83e7d-14b6-4352-bcaf-f3c673d61c93	0
002f5f3d-e6fe-4765-a7d3-1d07fec21539	0
65b30778-8c21-41ff-aa76-5e4f8030d12d	18.48
7e00bd6d-eaa5-4e7a-95fe-3c8f03427b3e	18.48
51937226-98db-4d5d-808b-6960c27d6eb3	0
7d4e53e1-85de-45fa-b417-0ce83aa61828	0
a8d4651d-42bc-40e0-8310-6c6e3fa70431	10.99
b9cd336e-57eb-46f8-b3bf-e2eb3a5ebd43	10.99
8f58f058-7bd5-4e1d-97ee-377890e10eba	28.38
4fe2fcac-f557-4505-9b42-7c01d6b30ffd	25.76
49cdb0fe-00b3-4ca1-b0d9-31e93c202185	25.86
a2cb3e9f-4a8a-4ac8-b12a-53bd5c792fbc	23.74
eb38ad18-ffbf-45eb-bf01-931deb98c1dc	20.72
d7c5d8df-4b3c-4734-84e5-c364fd626cf3	15.49
7b138125-e81c-4b00-b1ad-11ed620685cd	14.89
c1235fcf-82c6-4cf9-bd18-4a457b4f0a13	16.27
0724c17a-750b-4db4-af45-68b8103eec77	16.27
8fd7642f-4a31-4a13-95c8-f1ea1aa263ba	15.31
e09fb7d5-dd65-44db-a963-d07523215d84	25.33
5885ec0d-6af5-43c1-9978-d2389e40b95f	29.84
\.


--
-- Data for Name: seaql_migrations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.seaql_migrations (version, applied_at) FROM stdin;
m20220101_000001_main	1777560947
m20231010_232527_city_submission	1777560947
m20240202_004130_brokenspoke_analyzer_pipeline	1777560947
m20250529_151932_measure	1777560947
\.


--
-- Data for Name: state_region_crosswalk; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.state_region_crosswalk (state, region) FROM stdin;
Alabama	South
Alaska	Pacific
Arizona	Mountain
Arkansas	South
California	Pacific
Colorado	Mountain
Connecticut	New England
Delaware	Mid-Atlantic
District of Columbia	Mid-Atlantic
Florida	South
Georgia	South
Hawaii	Pacific
Idaho	Mountain
Illinois	Midwest
Indiana	Midwest
Iowa	Midwest
Kansas	Midwest
Kentucky	South
Louisiana	South
Maine	New England
Maryland	Mid-Atlantic
Massachusetts	New England
Michigan	Midwest
Minnesota	Midwest
Mississippi	South
Missouri	Midwest
Montana	Mountain
Nebraska	Midwest
Nevada	Mountain
New Hampshire	New England
New Jersey	Mid-Atlantic
New Mexico	Mountain
New York	Mid-Atlantic
North Carolina	South
North Dakota	Midwest
Ohio	Midwest
Oklahoma	South
Oregon	Pacific
Pennsylvania	Mid-Atlantic
Rhode Island	New England
South Carolina	South
South Dakota	Midwest
Tennessee	South
Texas	South
Utah	Mountain
Vermont	New England
Virginia	South
Washington	Pacific
West Virginia	South
Wisconsin	Midwest
Wyoming	Mountain
Puerto Rico	South
\.


--
-- Data for Name: submission; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.submission (id, first_name, last_name, occupation, organization, email, country, city, region, fips_code, consent, status, created_at) FROM stdin;
\.


--
-- Data for Name: summary; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.summary (id, city_id, created_at, pop_size, population, residential_speed_limit_override, score, version) FROM stdin;
bcbbe298-9cef-42e9-81bd-4e026b72f351	296149fe-5d0e-4ee4-ab8b-e1e548ad0bf2	2018-02-02 12:17:00+00	1	118620	\N	32	18.01
f838f8c1-9833-45c9-acdf-f55218c7c13f	296149fe-5d0e-4ee4-ab8b-e1e548ad0bf2	2019-02-15 07:50:00+00	1	120248	\N	32	19.01
0b8fc358-3e4d-44fd-8368-59f5cc9a7007	296149fe-5d0e-4ee4-ab8b-e1e548ad0bf2	2020-02-20 07:32:00+00	1	121230	\N	36	20.01
fb12b2d7-dfaa-46f3-b63c-4c7b05e7824e	296149fe-5d0e-4ee4-ab8b-e1e548ad0bf2	2021-02-25 23:01:00+00	1	122659	\N	36	21.01
f0702ab1-48f9-45dc-861e-3aed6c20b310	296149fe-5d0e-4ee4-ab8b-e1e548ad0bf2	2022-01-25 09:03:00+00	1	124342	\N	37	22.01
27795032-4c19-4b73-9466-87b1573a6212	296149fe-5d0e-4ee4-ab8b-e1e548ad0bf2	2023-04-02 21:44:00+00	1	126172	\N	36	23.01
e32e5c1e-72ce-4d1b-9a32-7467ac7448aa	296149fe-5d0e-4ee4-ab8b-e1e548ad0bf2	2023-08-24 10:17:00+00	1	126172	\N	36	23.02
24882ecf-dfbf-4eef-a36c-eec23648a8c2	296149fe-5d0e-4ee4-ab8b-e1e548ad0bf2	2023-12-07 14:19:00+00	1	126172	\N	36	23.03
14b557ed-19fb-4bf6-bab4-37a02ebf0b61	296149fe-5d0e-4ee4-ab8b-e1e548ad0bf2	2024-03-18 22:28:00+00	1	126172	\N	39	24.01
666bddf7-5e3b-4def-ad36-960f805877cb	296149fe-5d0e-4ee4-ab8b-e1e548ad0bf2	2025-03-24 23:15:26.968664+00	1	127200	\N	39	25.01
7eaf136b-9a57-435d-8591-483de5610b5a	a18348b9-fcbd-46de-975c-ee74119519c3	2021-07-15 08:54:00+00	1	113117	\N	22	21.01
2519af81-b920-4707-bed6-8e56d93bcc74	a18348b9-fcbd-46de-975c-ee74119519c3	2022-01-22 12:26:00+00	1	109376	\N	22	22.01
9b43f089-2932-45da-b924-613e643ca7f2	a18348b9-fcbd-46de-975c-ee74119519c3	2022-01-22 13:18:00+00	1	109376	\N	22	22.02
bb290e88-d405-45d8-b604-6eb5e3a0e65c	a18348b9-fcbd-46de-975c-ee74119519c3	2022-06-10 06:51:00+00	1	109376	\N	22	22.03
fb09d896-e363-4be0-9c4c-255acd0e2279	a18348b9-fcbd-46de-975c-ee74119519c3	2023-04-01 07:31:00+00	1	110114	\N	23	23.01
42cf1436-f053-44bd-9859-550ea313ec95	a18348b9-fcbd-46de-975c-ee74119519c3	2023-08-19 08:11:00+00	1	110114	\N	23	23.02
ae7bcb6d-69cc-4ece-9acb-c64730079869	a18348b9-fcbd-46de-975c-ee74119519c3	2023-12-04 14:42:00+00	1	110114	\N	22	23.03
a79deacf-525f-4b3c-800b-0dd9b8d66b33	a18348b9-fcbd-46de-975c-ee74119519c3	2024-03-05 17:23:00+00	1	110114	\N	22	24.01
96a55ca7-2c80-42ad-acd3-92e7917ffd6d	a18348b9-fcbd-46de-975c-ee74119519c3	2025-03-13 18:52:43.796763+00	1	110404	\N	22	25.01
5ed0f710-edce-4b32-8512-acee72d6956e	bf5f3e41-f394-4c0b-9550-060802adc6fa	2023-12-05 14:01:00+00	0	6030	\N	32	23.01
01674cb0-7f28-41b8-be65-f9ad36bb9272	bf5f3e41-f394-4c0b-9550-060802adc6fa	2024-03-10 14:52:00+00	0	6030	\N	34	24.01
380a7ec6-d1c8-4723-b252-cfa5d06a33e3	bf5f3e41-f394-4c0b-9550-060802adc6fa	2025-03-22 00:58:08.03203+00	0	5984	\N	36	25.01
a9282493-85d7-4262-8a20-10eec8be2576	5180283b-f201-49af-9e7a-a6fc243ef2df	2021-07-15 08:54:00+00	0	36495	\N	24	21.01
8569094d-b182-40a2-995a-8219c4a8fcfc	5180283b-f201-49af-9e7a-a6fc243ef2df	2022-01-24 08:34:00+00	0	37074	\N	24	22.01
48ee9912-28fe-4582-8c4d-23c2ec9e19ac	5180283b-f201-49af-9e7a-a6fc243ef2df	2023-04-03 10:46:00+00	0	37314	\N	23	23.01
66e766ad-82aa-4b1a-865f-9af40886866a	5180283b-f201-49af-9e7a-a6fc243ef2df	2023-08-27 14:08:00+00	0	37314	\N	23	23.02
201e3ed8-cdc8-4d0a-ac48-43f34bba6157	5180283b-f201-49af-9e7a-a6fc243ef2df	2023-12-06 11:22:00+00	0	37314	\N	22	23.03
c222a999-3b81-43f4-b8ff-1edf62492c02	5180283b-f201-49af-9e7a-a6fc243ef2df	2024-03-12 16:06:00+00	0	37314	\N	23	24.01
b33f8354-dfce-4a16-ab27-22297226e01f	5180283b-f201-49af-9e7a-a6fc243ef2df	2025-03-22 03:04:35.389279+00	0	37559	\N	24	25.01
41430704-c565-47a3-b97a-0b9950be7d87	d755b6e8-6dee-4cae-9ce8-3662be60f776	2025-03-13 18:52:46.943757+00	0	10680	\N	34	25.01
856be1f6-eb45-45ea-b1bf-ffb5dd0643e0	3aea277c-9c05-44e2-87a5-6a1b900bf0d2	2021-03-25 09:55:00+00	1	159458	\N	32	21.01
cd2909ff-81c9-4006-bdda-579a233c3911	3aea277c-9c05-44e2-87a5-6a1b900bf0d2	2022-03-28 16:00:00+00	1	175383	\N	36	22.01
aa938808-37fc-429c-b79f-b6295b1f9917	3aea277c-9c05-44e2-87a5-6a1b900bf0d2	2023-04-13 08:15:00+00	1	175383	\N	36	23.01
87b533b6-b5b0-4a7b-a8eb-93de2a8ca054	3aea277c-9c05-44e2-87a5-6a1b900bf0d2	2024-04-04 13:25:00+00	1	175383	\N	44	24.01
1eeab18e-a24c-4321-8f3f-14f52f9360be	3aea277c-9c05-44e2-87a5-6a1b900bf0d2	2025-04-21 03:40:23.247091+00	1	200555	\N	48	25.01
c95dfd2d-f989-4c10-a9d6-c8450a488646	419779dc-65f1-410a-a55d-42c379e252de	2023-08-27 14:08:00+00	0	36933	\N	25	23.01
6b470afd-0f87-4bdd-a9b2-1e3be794a053	419779dc-65f1-410a-a55d-42c379e252de	2023-12-06 11:22:00+00	0	36933	\N	25	23.02
aede142b-f3d3-45a6-bf7c-66c05b33f5a3	419779dc-65f1-410a-a55d-42c379e252de	2024-03-12 16:06:00+00	0	36933	\N	28	24.01
cf05a5f2-55f9-4d32-a011-585b4c76b5cd	419779dc-65f1-410a-a55d-42c379e252de	2025-03-22 03:04:35.086807+00	0	36819	\N	30	25.01
7e354518-6652-4685-a9ab-bf385f184683	8fbe8758-1dc7-4da1-8926-34a6baf5858a	2022-02-01 17:08:00+00	0	39927	\N	12	22.01
d106416f-849b-4b60-812c-a9633faa1b91	8fbe8758-1dc7-4da1-8926-34a6baf5858a	2023-04-01 19:05:00+00	0	39873	\N	11	23.01
5b6a2e81-ed02-4052-9428-b392c09d153d	8fbe8758-1dc7-4da1-8926-34a6baf5858a	2023-08-21 10:59:00+00	0	39873	\N	12	23.02
8685dc55-b4aa-4977-b9ef-268b5deb872e	8fbe8758-1dc7-4da1-8926-34a6baf5858a	2023-12-06 14:50:00+00	0	39873	\N	12	23.03
ee2c925a-a9cd-4bde-8beb-9b3868c5d4b8	8fbe8758-1dc7-4da1-8926-34a6baf5858a	2024-03-12 09:13:00+00	0	39873	\N	12	24.01
a6c23082-a0fc-460b-87a7-bf2d6a8fa6d7	8fbe8758-1dc7-4da1-8926-34a6baf5858a	2025-03-26 18:51:33.67813+00	0	39871	\N	19	25.01
f2b65b6c-fb24-4b11-a44c-5d8c7d4c5b69	df866685-6784-4f95-bcab-8c9438c57630	2022-10-07 12:06:00+00	0	680	\N	10	22.01
1dce7b76-ea70-47eb-96da-7a88f61d4470	df866685-6784-4f95-bcab-8c9438c57630	2023-04-01 14:23:00+00	0	655	\N	10	23.01
52d83e7d-14b6-4352-bcaf-f3c673d61c93	df866685-6784-4f95-bcab-8c9438c57630	2023-08-19 07:54:00+00	0	655	\N	10	23.02
002f5f3d-e6fe-4765-a7d3-1d07fec21539	df866685-6784-4f95-bcab-8c9438c57630	2023-12-06 07:57:00+00	0	655	\N	10	23.03
65b30778-8c21-41ff-aa76-5e4f8030d12d	df866685-6784-4f95-bcab-8c9438c57630	2024-03-10 14:49:00+00	0	655	\N	11	24.01
7e00bd6d-eaa5-4e7a-95fe-3c8f03427b3e	df866685-6784-4f95-bcab-8c9438c57630	2025-03-15 13:52:43.092106+00	0	576	\N	11	25.01
51937226-98db-4d5d-808b-6960c27d6eb3	7a645a59-1477-4fc0-83ad-75f55129b09f	2023-08-21 10:51:00+00	0	1462	\N	18	23.01
7d4e53e1-85de-45fa-b417-0ce83aa61828	7a645a59-1477-4fc0-83ad-75f55129b09f	2023-12-06 14:45:00+00	0	1462	\N	18	23.02
a8d4651d-42bc-40e0-8310-6c6e3fa70431	7a645a59-1477-4fc0-83ad-75f55129b09f	2024-03-06 14:32:00+00	0	1462	\N	17	24.01
b9cd336e-57eb-46f8-b3bf-e2eb3a5ebd43	7a645a59-1477-4fc0-83ad-75f55129b09f	2025-03-23 20:45:26.24385+00	0	1235	\N	17	25.01
8f58f058-7bd5-4e1d-97ee-377890e10eba	4ccfcf6b-17c9-4006-b1cd-7e2829d85d54	2017-04-29 06:58:00+00	1	175462	\N	23	17.01
4fe2fcac-f557-4505-9b42-7c01d6b30ffd	4ccfcf6b-17c9-4006-b1cd-7e2829d85d54	2017-07-18 11:17:00+00	1	175462	\N	22	17.02
49cdb0fe-00b3-4ca1-b0d9-31e93c202185	4ccfcf6b-17c9-4006-b1cd-7e2829d85d54	2018-01-29 07:56:00+00	1	176291	\N	21	18.01
a2cb3e9f-4a8a-4ac8-b12a-53bd5c792fbc	4ccfcf6b-17c9-4006-b1cd-7e2829d85d54	2019-02-14 21:43:00+00	1	177365	\N	21	19.01
eb38ad18-ffbf-45eb-bf01-931deb98c1dc	4ccfcf6b-17c9-4006-b1cd-7e2829d85d54	2020-02-19 08:36:00+00	1	179690	\N	20	20.01
d7c5d8df-4b3c-4734-84e5-c364fd626cf3	4ccfcf6b-17c9-4006-b1cd-7e2829d85d54	2021-02-26 16:33:00+00	1	181370	\N	20	21.01
7b138125-e81c-4b00-b1ad-11ed620685cd	4ccfcf6b-17c9-4006-b1cd-7e2829d85d54	2022-01-24 10:17:00+00	1	180353	\N	20	22.01
c1235fcf-82c6-4cf9-bd18-4a457b4f0a13	4ccfcf6b-17c9-4006-b1cd-7e2829d85d54	2023-04-02 22:26:00+00	1	181288	\N	20	23.01
0724c17a-750b-4db4-af45-68b8103eec77	4ccfcf6b-17c9-4006-b1cd-7e2829d85d54	2023-08-19 08:30:00+00	1	181288	\N	21	23.02
8fd7642f-4a31-4a13-95c8-f1ea1aa263ba	4ccfcf6b-17c9-4006-b1cd-7e2829d85d54	2023-12-06 08:32:00+00	1	181288	\N	20	23.03
e09fb7d5-dd65-44db-a963-d07523215d84	4ccfcf6b-17c9-4006-b1cd-7e2829d85d54	2024-03-13 11:25:00+00	1	181288	\N	22	24.01
5885ec0d-6af5-43c1-9978-d2389e40b95f	4ccfcf6b-17c9-4006-b1cd-7e2829d85d54	2025-03-21 17:25:30.378391+00	1	182832	\N	24	25.01
\.


--
-- Data for Name: transit; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.transit (id, score) FROM stdin;
bcbbe298-9cef-42e9-81bd-4e026b72f351	48.1
f838f8c1-9833-45c9-acdf-f55218c7c13f	47.72
0b8fc358-3e4d-44fd-8368-59f5cc9a7007	47.72
fb12b2d7-dfaa-46f3-b63c-4c7b05e7824e	47.85
f0702ab1-48f9-45dc-861e-3aed6c20b310	48.61
27795032-4c19-4b73-9466-87b1573a6212	48.61
e32e5c1e-72ce-4d1b-9a32-7467ac7448aa	48.58
24882ecf-dfbf-4eef-a36c-eec23648a8c2	48.58
14b557ed-19fb-4bf6-bab4-37a02ebf0b61	49.49
666bddf7-5e3b-4def-ad36-960f805877cb	42.06
7eaf136b-9a57-435d-8591-483de5610b5a	0.79
2519af81-b920-4707-bed6-8e56d93bcc74	0.79
9b43f089-2932-45da-b924-613e643ca7f2	0.79
bb290e88-d405-45d8-b604-6eb5e3a0e65c	0.79
fb09d896-e363-4be0-9c4c-255acd0e2279	0.79
42cf1436-f053-44bd-9859-550ea313ec95	0.79
ae7bcb6d-69cc-4ece-9acb-c64730079869	0.79
a79deacf-525f-4b3c-800b-0dd9b8d66b33	0.79
96a55ca7-2c80-42ad-acd3-92e7917ffd6d	0.79
5ed0f710-edce-4b32-8512-acee72d6956e	0
01674cb0-7f28-41b8-be65-f9ad36bb9272	0
380a7ec6-d1c8-4723-b252-cfa5d06a33e3	0
a9282493-85d7-4262-8a20-10eec8be2576	0
8569094d-b182-40a2-995a-8219c4a8fcfc	0
48ee9912-28fe-4582-8c4d-23c2ec9e19ac	0
66e766ad-82aa-4b1a-865f-9af40886866a	0
201e3ed8-cdc8-4d0a-ac48-43f34bba6157	0
c222a999-3b81-43f4-b8ff-1edf62492c02	0
b33f8354-dfce-4a16-ab27-22297226e01f	0
41430704-c565-47a3-b97a-0b9950be7d87	0
856be1f6-eb45-45ea-b1bf-ffb5dd0643e0	12.95
cd2909ff-81c9-4006-bdda-579a233c3911	24.05
aa938808-37fc-429c-b79f-b6295b1f9917	24.05
87b533b6-b5b0-4a7b-a8eb-93de2a8ca054	31.09
1eeab18e-a24c-4321-8f3f-14f52f9360be	34.25
c95dfd2d-f989-4c10-a9d6-c8450a488646	0
6b470afd-0f87-4bdd-a9b2-1e3be794a053	0
aede142b-f3d3-45a6-bf7c-66c05b33f5a3	0
cf05a5f2-55f9-4d32-a011-585b4c76b5cd	0
7e354518-6652-4685-a9ab-bf385f184683	18.48
d106416f-849b-4b60-812c-a9633faa1b91	15.99
5b6a2e81-ed02-4052-9428-b392c09d153d	22.45
8685dc55-b4aa-4977-b9ef-268b5deb872e	12.63
ee2c925a-a9cd-4bde-8beb-9b3868c5d4b8	12.06
a6c23082-a0fc-460b-87a7-bf2d6a8fa6d7	14.77
f2b65b6c-fb24-4b11-a44c-5d8c7d4c5b69	0
1dce7b76-ea70-47eb-96da-7a88f61d4470	0
52d83e7d-14b6-4352-bcaf-f3c673d61c93	0
002f5f3d-e6fe-4765-a7d3-1d07fec21539	0
65b30778-8c21-41ff-aa76-5e4f8030d12d	0
7e00bd6d-eaa5-4e7a-95fe-3c8f03427b3e	0
51937226-98db-4d5d-808b-6960c27d6eb3	0
7d4e53e1-85de-45fa-b417-0ce83aa61828	0
a8d4651d-42bc-40e0-8310-6c6e3fa70431	0
b9cd336e-57eb-46f8-b3bf-e2eb3a5ebd43	0
8f58f058-7bd5-4e1d-97ee-377890e10eba	1.71
4fe2fcac-f557-4505-9b42-7c01d6b30ffd	1.71
49cdb0fe-00b3-4ca1-b0d9-31e93c202185	1.71
a2cb3e9f-4a8a-4ac8-b12a-53bd5c792fbc	1.1
eb38ad18-ffbf-45eb-bf01-931deb98c1dc	1.11
d7c5d8df-4b3c-4734-84e5-c364fd626cf3	1.34
7b138125-e81c-4b00-b1ad-11ed620685cd	1.98
c1235fcf-82c6-4cf9-bd18-4a457b4f0a13	9.4
0724c17a-750b-4db4-af45-68b8103eec77	11.71
8fd7642f-4a31-4a13-95c8-f1ea1aa263ba	11.71
e09fb7d5-dd65-44db-a963-d07523215d84	10.3
5885ec0d-6af5-43c1-9978-d2389e40b95f	10.21
\.


--
-- Data for Name: us_state; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.us_state (name, abbrev, fips_code, speed_limit) FROM stdin;
Alabama	AL	01	25
Alaska	AK	02	25
Arizona	AZ	04	25
Arkansas	AR	05	30
California	CA	06	25
Colorado	CO	08	30
Connecticut	CT	09	25
Delaware	DE	10	25
District of Columbia	DC	11	20
Florida	FL	12	30
Georgia	GA	13	30
Hawaii	HI	15	25
Idaho	ID	16	35
Illinois	IL	17	30
Indiana	IN	18	30
Iowa	IA	19	25
Kansas	KS	20	30
Kentucky	KY	21	35
Louisiana	LA	22	25
Maine	ME	23	25
Maryland	MD	24	30
Massachusetts	MA	25	25
Michigan	MI	26	25
Minnesota	MN	27	30
Mississippi	MS	28	25
Missouri	MO	29	25
Montana	MT	30	25
Nebraska	NE	31	25
Nevada	NV	32	25
New Hampshire	NH	33	30
New Jersey	NJ	34	25
New Mexico	NM	35	30
New York	NY	36	20
North Carolina	NC	37	35
North Dakota	ND	38	25
Ohio	OH	39	25
Oklahoma	OK	40	25
Oregon	OR	41	25
Pennsylvania	PA	42	25
Rhode Island	RI	44	25
South Carolina	SC	45	30
South Dakota	SD	46	25
Tennessee	TN	47	25
Texas	TX	48	30
Utah	UT	49	25
Vermont	VT	50	25
Virginia	VA	51	25
Washington	WA	53	25
West Virginia	WV	54	25
Wisconsin	WI	55	25
Wyoming	WY	56	30
Puerto Rico	PR	77	25
\.


--
-- Name: fargate_price_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.fargate_price_id_seq', 1, true);


--
-- Name: submission_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.submission_id_seq', 1, false);


--
-- Name: approval_status approval_status_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.approval_status
    ADD CONSTRAINT approval_status_pkey PRIMARY KEY (status);


--
-- Name: bike_lane_type bike_lane_type_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bike_lane_type
    ADD CONSTRAINT bike_lane_type_pkey PRIMARY KEY (name);


--
-- Name: bna_pipeline bna_pipeline_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bna_pipeline
    ADD CONSTRAINT bna_pipeline_pkey PRIMARY KEY (state_machine_id);


--
-- Name: bna_pipeline_status bna_pipeline_status_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bna_pipeline_status
    ADD CONSTRAINT bna_pipeline_status_pkey PRIMARY KEY (status);


--
-- Name: bna_pipeline_step bna_pipeline_step_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bna_pipeline_step
    ADD CONSTRAINT bna_pipeline_step_pkey PRIMARY KEY (step);


--
-- Name: bna_region bna_region_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bna_region
    ADD CONSTRAINT bna_region_pkey PRIMARY KEY (name);


--
-- Name: city city_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.city
    ADD CONSTRAINT city_id_key UNIQUE (id);


--
-- Name: city city_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.city
    ADD CONSTRAINT city_pkey PRIMARY KEY (country, state, name);


--
-- Name: core_services core_services_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.core_services
    ADD CONSTRAINT core_services_pkey PRIMARY KEY (id);


--
-- Name: country country_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.country
    ADD CONSTRAINT country_pkey PRIMARY KEY (name);


--
-- Name: fargate_price fargate_price_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.fargate_price
    ADD CONSTRAINT fargate_price_pkey PRIMARY KEY (id);


--
-- Name: infrastructure infrastructure_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.infrastructure
    ADD CONSTRAINT infrastructure_pkey PRIMARY KEY (id);


--
-- Name: measure measure_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.measure
    ADD CONSTRAINT measure_pkey PRIMARY KEY (id);


--
-- Name: opportunity opportunity_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.opportunity
    ADD CONSTRAINT opportunity_pkey PRIMARY KEY (id);


--
-- Name: people people_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.people
    ADD CONSTRAINT people_pkey PRIMARY KEY (id);


--
-- Name: recreation recreation_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.recreation
    ADD CONSTRAINT recreation_pkey PRIMARY KEY (id);


--
-- Name: retail retail_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.retail
    ADD CONSTRAINT retail_pkey PRIMARY KEY (id);


--
-- Name: seaql_migrations seaql_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.seaql_migrations
    ADD CONSTRAINT seaql_migrations_pkey PRIMARY KEY (version);


--
-- Name: state_region_crosswalk state_region_crosswalk_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.state_region_crosswalk
    ADD CONSTRAINT state_region_crosswalk_pkey PRIMARY KEY (state, region);


--
-- Name: submission submission_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.submission
    ADD CONSTRAINT submission_pkey PRIMARY KEY (id);


--
-- Name: summary summary_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.summary
    ADD CONSTRAINT summary_pkey PRIMARY KEY (id);


--
-- Name: transit transit_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.transit
    ADD CONSTRAINT transit_pkey PRIMARY KEY (id);


--
-- Name: us_state us_state_abbrev_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.us_state
    ADD CONSTRAINT us_state_abbrev_key UNIQUE (abbrev);


--
-- Name: us_state us_state_fips_code_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.us_state
    ADD CONSTRAINT us_state_fips_code_key UNIQUE (fips_code);


--
-- Name: us_state us_state_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.us_state
    ADD CONSTRAINT us_state_pkey PRIMARY KEY (name);


--
-- Name: city_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX city_id_idx ON public.city USING btree (id);


--
-- Name: state_region_crosswalk_region_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX state_region_crosswalk_region_idx ON public.state_region_crosswalk USING btree (region);


--
-- Name: state_region_crosswalk_state_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX state_region_crosswalk_state_idx ON public.state_region_crosswalk USING btree (state);


--
-- Name: summary_city_id_created_at_version_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX summary_city_id_created_at_version_idx ON public.summary USING btree (city_id, created_at DESC, version DESC);


--
-- Name: us_state_abbrev_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX us_state_abbrev_idx ON public.us_state USING btree (abbrev);


--
-- Name: us_state_fips_code_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX us_state_fips_code_idx ON public.us_state USING btree (fips_code);


--
-- Name: bna_pipeline bna_pipeline_fargate_price_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bna_pipeline
    ADD CONSTRAINT bna_pipeline_fargate_price_id_fkey FOREIGN KEY (fargate_price_id) REFERENCES public.fargate_price(id);


--
-- Name: bna_pipeline bna_pipeline_status_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bna_pipeline
    ADD CONSTRAINT bna_pipeline_status_fkey FOREIGN KEY (status) REFERENCES public.bna_pipeline_status(status);


--
-- Name: bna_pipeline bna_pipeline_step_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bna_pipeline
    ADD CONSTRAINT bna_pipeline_step_fkey FOREIGN KEY (step) REFERENCES public.bna_pipeline_step(step);


--
-- Name: city city_country_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.city
    ADD CONSTRAINT city_country_fkey FOREIGN KEY (country) REFERENCES public.country(name);


--
-- Name: core_services core_services_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.core_services
    ADD CONSTRAINT core_services_id_fkey FOREIGN KEY (id) REFERENCES public.summary(id) ON DELETE CASCADE;


--
-- Name: infrastructure infrastructure_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.infrastructure
    ADD CONSTRAINT infrastructure_id_fkey FOREIGN KEY (id) REFERENCES public.summary(id) ON DELETE CASCADE;


--
-- Name: measure measure_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.measure
    ADD CONSTRAINT measure_id_fkey FOREIGN KEY (id) REFERENCES public.summary(id) ON DELETE CASCADE;


--
-- Name: opportunity opportunity_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.opportunity
    ADD CONSTRAINT opportunity_id_fkey FOREIGN KEY (id) REFERENCES public.summary(id) ON DELETE CASCADE;


--
-- Name: people people_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.people
    ADD CONSTRAINT people_id_fkey FOREIGN KEY (id) REFERENCES public.summary(id) ON DELETE CASCADE;


--
-- Name: recreation recreation_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.recreation
    ADD CONSTRAINT recreation_id_fkey FOREIGN KEY (id) REFERENCES public.summary(id) ON DELETE CASCADE;


--
-- Name: retail retail_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.retail
    ADD CONSTRAINT retail_id_fkey FOREIGN KEY (id) REFERENCES public.summary(id) ON DELETE CASCADE;


--
-- Name: state_region_crosswalk state_region_crosswalk_region_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.state_region_crosswalk
    ADD CONSTRAINT state_region_crosswalk_region_fkey FOREIGN KEY (region) REFERENCES public.bna_region(name);


--
-- Name: state_region_crosswalk state_region_crosswalk_state_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.state_region_crosswalk
    ADD CONSTRAINT state_region_crosswalk_state_fkey FOREIGN KEY (state) REFERENCES public.us_state(name);


--
-- Name: submission submission_country_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.submission
    ADD CONSTRAINT submission_country_fkey FOREIGN KEY (country) REFERENCES public.country(name);


--
-- Name: submission submission_status_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.submission
    ADD CONSTRAINT submission_status_fkey FOREIGN KEY (status) REFERENCES public.approval_status(status);


--
-- Name: summary summary_city_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.summary
    ADD CONSTRAINT summary_city_id_fkey FOREIGN KEY (city_id) REFERENCES public.city(id) ON DELETE CASCADE;


--
-- Name: transit transit_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.transit
    ADD CONSTRAINT transit_id_fkey FOREIGN KEY (id) REFERENCES public.summary(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

\unrestrict ExtkqABOWrvx0DUx9HgDijXj8f8FIkZV0M1JJ2wUGeXfQTiEqA2MdbWllxtsmjZ

