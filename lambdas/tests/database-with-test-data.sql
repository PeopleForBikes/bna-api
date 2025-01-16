--
-- PostgreSQL database dump
--

-- Dumped from database version 15.2 (Debian 15.2-1.pgdg110+1)
-- Dumped by pg_dump version 17.2 (Homebrew)

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
-- Name: census; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.census (
    id integer NOT NULL,
    city_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    fips_code character varying NOT NULL,
    pop_size integer NOT NULL,
    population integer NOT NULL
);


ALTER TABLE public.census OWNER TO postgres;

--
-- Name: census_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.census_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.census_id_seq OWNER TO postgres;

--
-- Name: census_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.census_id_seq OWNED BY public.census.id;


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
    speed_limit integer,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone
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
-- Name: speed_limit; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.speed_limit (
    id integer NOT NULL,
    city_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    residential integer NOT NULL
);


ALTER TABLE public.speed_limit OWNER TO postgres;

--
-- Name: speed_limit_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.speed_limit_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.speed_limit_id_seq OWNER TO postgres;

--
-- Name: speed_limit_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.speed_limit_id_seq OWNED BY public.speed_limit.id;


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
-- Name: census id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.census ALTER COLUMN id SET DEFAULT nextval('public.census_id_seq'::regclass);


--
-- Name: fargate_price id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.fargate_price ALTER COLUMN id SET DEFAULT nextval('public.fargate_price_id_seq'::regclass);


--
-- Name: speed_limit id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.speed_limit ALTER COLUMN id SET DEFAULT nextval('public.speed_limit_id_seq'::regclass);


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
-- Data for Name: census; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.census (id, city_id, created_at, fips_code, pop_size, population) FROM stdin;
1	58f1f419-ac98-4f12-9f4d-d78939773041	2024-03-08 15:30:00+00	5527300	0	36513
2	6ba8c886-419a-4880-a749-b7e4246bb78c	2024-03-05 17:23:00+00	623042	0	26519
3	06b9181f-f879-40d5-b531-c239caa8ccd9	2023-08-23 13:59:00+00	3737680	0	24473
4	09c049b6-213b-405c-bc4e-178346ff814d	2021-02-25 13:26:00+00	1734722	0	29596
5	06118205-6e16-46da-bc9c-459860189e8f	2024-03-20 17:11:00+00	4233112	0	3490
6	4ccfcf6b-17c9-4006-b1cd-7e2829d85d54	2023-12-06 08:32:00+00	4714000	1	181288
7	4febf356-c079-41c9-81b4-e930dcf2daac	2017-04-29 06:59:00+00	1782400	0	9543
8	712846c3-d432-480b-b582-d47730cf90bf	2017-04-29 06:58:00+00	2053775	1	183775
9	3fa975ec-55af-4a63-addf-a36b920fc9a7	2024-03-14 09:22:00+00	5471212	0	10753
10	fbdeaa5e-92d7-43b7-b00b-915b8c10c9a7	2022-02-01 19:14:00+00	5335940	1	91656
11	a1fe143b-6253-40d8-bfb0-7bfaeccee6c4	2023-04-17 16:59:00+00	9900139	2	338577
12	6ef16bd9-0759-447d-b7af-744888b824ca	2024-04-04 13:13:00+00	9900063	0	25599
13	a6c14f18-ff74-42fb-9324-72f2c8d0fb66	2024-04-01 15:00:00+00	9900184	1	186434
14	bbd47ea1-3ca9-41b6-803d-ab7c9505e768	2023-01-07 21:28:00+00	9900233	2	1476491
15	daffa2db-4980-4ddd-8bf2-4bf79ef09e10	2024-04-03 16:47:00+00	9900003	0	37396
16	f249b76f-3db0-4641-b2d7-b65aa69ee229	2023-04-12 16:18:00+00	9900080	1	63116
17	8f11086a-b44a-44cf-a755-1fc9c257a48d	2021-03-24 12:30:00+00	9900076	1	85792
18	bd1f74b3-bcc8-44c2-959f-f959accc3712	2024-04-12 10:49:00+00	9900051	2	446731
19	3bd1329c-4f97-4278-beae-c025a6a1ea66	2023-04-18 09:34:00+00	9900234	1	214715
20	bfdde387-2429-490a-95f0-4e719ef7aa78	2022-03-28 16:00:00+00	9900096	2	717961
\.


--
-- Data for Name: city; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.city (id, country, state, name, latitude, longitude, region, state_abbrev, speed_limit, created_at, updated_at) FROM stdin;
6ef16bd9-0759-447d-b7af-744888b824ca	Canada	British Columbia	Courtenay	49.6841	-124.9904	Canada	BC	\N	2024-04-04 13:13:00+00	\N
3bd1329c-4f97-4278-beae-c025a6a1ea66	Netherlands	Flevoland	Almere	52.3508	5.2647	Netherlands	FL	\N	2023-04-18 09:34:00+00	\N
712846c3-d432-480b-b582-d47730cf90bf	United States	Kansas	Overland Park	38.889	-94.6906	Midwest	KS	25	2017-04-29 06:58:00+00	\N
09c049b6-213b-405c-bc4e-178346ff814d	United States	Illinois	Highland Park	42.1824	-87.8108	Midwest	IL	25	2021-02-25 13:26:00+00	\N
a1fe143b-6253-40d8-bfb0-7bfaeccee6c4	Spain	Alicante	Alicante	38.346	-0.4907	Spain	A	\N	2023-04-17 16:59:00+00	\N
3fa975ec-55af-4a63-addf-a36b920fc9a7	United States	West Virginia	St. Albans	38.3776	-81.8193	South	WV	\N	2024-03-14 09:22:00+00	\N
bbd47ea1-3ca9-41b6-803d-ab7c9505e768	Mexico	Jalisco	Zapopan	20.7203	-103.3919	Mexico	JAL	\N	2023-01-07 21:28:00+00	\N
06118205-6e16-46da-bc9c-459860189e8f	United States	Pennsylvania	Hatfield	40.2771	-75.2989	Mid-Atlantic	PA	\N	2024-03-20 17:11:00+00	\N
bfdde387-2429-490a-95f0-4e719ef7aa78	Canada	Ontario	Mississauga	43.589	-79.6441	Canada	ON	\N	2022-03-28 16:00:00+00	\N
06b9181f-f879-40d5-b531-c239caa8ccd9	United States	North Carolina	Leland	34.223	-78.0447	South	NC	\N	2023-08-23 13:59:00+00	\N
4febf356-c079-41c9-81b4-e930dcf2daac	United States	Illinois	Winfield	41.8785	-88.1502	Midwest	IL	\N	2017-04-29 06:59:00+00	\N
bd1f74b3-bcc8-44c2-959f-f959accc3712	Brazil	Rio de Janeiro	Belford Roxo	-22.7606	-43.4037	Brazil	RJ	\N	2024-04-12 10:49:00+00	\N
fbdeaa5e-92d7-43b7-b00b-915b8c10c9a7	United States	Washington	Kirkland	47.6967	-122.2042	Pacific	WA	\N	2022-02-01 19:14:00+00	\N
a6c14f18-ff74-42fb-9324-72f2c8d0fb66	England	Greater London	Lambeth	51.4581	-0.1237	England	\N	\N	2024-04-01 15:00:00+00	\N
58f1f419-ac98-4f12-9f4d-d78939773041	United States	Wisconsin	Franklin	42.8839	-88.0115	Midwest	WI	\N	2024-03-08 15:30:00+00	\N
6ba8c886-419a-4880-a749-b7e4246bb78c	United States	California	Eureka	40.7934	-124.1552	Pacific	CA	\N	2024-03-05 17:23:00+00	\N
4ccfcf6b-17c9-4006-b1cd-7e2829d85d54	United States	Tennessee	Chattanooga	35.066	-85.2484	South	TN	25	2023-12-06 08:32:00+00	\N
daffa2db-4980-4ddd-8bf2-4bf79ef09e10	Australia	New South Wales	Bathurst	-33.4107	149.5783	Australia	NSW	\N	2024-04-03 16:47:00+00	\N
8f11086a-b44a-44cf-a755-1fc9c257a48d	Canada	British Columbia	Victoria	48.4284	-123.3656	Canada	BC	\N	2021-03-24 12:30:00+00	\N
f249b76f-3db0-4641-b2d7-b65aa69ee229	Canada	New Brunswick	Fredericton	45.9636	-66.6431	Canada	NB	\N	2023-04-12 16:18:00+00	\N
\.


--
-- Data for Name: core_services; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.core_services (id, dentists, doctors, grocery, hospitals, pharmacies, score, social_services) FROM stdin;
fc8f1d05-55e8-42cf-967e-002056ba050e	25.08	18.19	24.16	17.08	16.58	19.03	11.79
f2058cdd-dea8-491b-80cf-11d08caceac8	9.71	8.13	37.4	4.01	24.07	17.34	14.55
da7327c5-34c9-44bc-bee4-31b6ad5bf1d2	0	7.11	21.07	14.06	22.8	18.95	35.16
aa8cf39e-93d7-4be0-99a4-424a4c7613b6	2.76	12.44	13.62	21.61	8.55	11.35	0
23fe492a-cef7-4e3b-997f-3116bd53c7aa	63.56	0	39.29	0	7.5	42.58	57.45
8fd7642f-4a31-4a13-95c8-f1ea1aa263ba	14.69	12.54	20.55	14.63	15.93	15.1	9.75
30597289-57c0-4f26-acc8-aa88b3859d0e	0	0	0.72	19.17	24.74	8.65	0
9b3f28d6-ecf0-4046-abe3-3657d6ea30ae	0	20.15	38.06	25.88	17.95	23.21	2.46
98d8327f-f0be-4486-93fd-fb238582889c	18.67	0.92	39.33	0	37.93	24.12	0
723d90e9-a5e3-44cc-9957-f480f928cf02	13.35	21.93	32.91	13.84	21.45	21.14	15.22
61d735fa-49d1-4b66-87d8-4a7e765aaf8b	67.88	71.49	78.37	56.66	71.63	68.21	60.27
ec300bdb-6d8f-4ff4-b3a6-ce6d3c2fab54	36.69	48.93	47.42	25.25	41.42	38.62	27.45
d65a350a-b918-4d35-8cf0-99c79af95269	81.2	80.52	86.27	75.67	81.54	81.42	82.26
b1ef6021-c779-4373-9147-6f6e442bbdab	32.68	38.39	79.7	56.43	59.43	55.7	50.69
8b9a93ea-63f0-4d01-8caa-2bbd3f0653e3	15.75	24.9	25.98	11.21	20.72	22.86	36.66
a66fb30f-7d80-4b27-90c6-4f6b0723ab89	32.98	49.86	56.09	55.56	48.31	51.91	57.81
5d224df6-262a-41b2-adb0-530f1909e896	10.41	17.07	39.61	10.91	35.94	23.95	25.42
3bc70777-1f75-479f-9eb9-7b639881ba19	0	35.04	73.87	50.07	41.78	52.89	0
36a29ed3-81c7-465b-a7f5-5e44a6a99cbf	80.16	83.21	86.52	84.57	79.36	82.58	76.26
fff6c9d5-5828-4c71-8bb7-5c462ff5cf9e	57.26	52.41	62.76	10.08	55.1	44.79	35.74
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
1	0.000038	2025-01-14 21:29:23.95619+00
\.


--
-- Data for Name: infrastructure; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.infrastructure (id, low_stress_miles, high_stress_miles) FROM stdin;
fc8f1d05-55e8-42cf-967e-002056ba050e	360	134.7
f2058cdd-dea8-491b-80cf-11d08caceac8	184.2	100.3
da7327c5-34c9-44bc-bee4-31b6ad5bf1d2	20.4	321.4
aa8cf39e-93d7-4be0-99a4-424a4c7613b6	238.7	79.5
23fe492a-cef7-4e3b-997f-3116bd53c7aa	15	6.8
8fd7642f-4a31-4a13-95c8-f1ea1aa263ba	2150.4	683.4
30597289-57c0-4f26-acc8-aa88b3859d0e	0	0
9b3f28d6-ecf0-4046-abe3-3657d6ea30ae	0	0
98d8327f-f0be-4486-93fd-fb238582889c	103	28.8
723d90e9-a5e3-44cc-9957-f480f928cf02	530.3	134.2
61d735fa-49d1-4b66-87d8-4a7e765aaf8b	1256.9	294.9
ec300bdb-6d8f-4ff4-b3a6-ce6d3c2fab54	86.2	208.8
d65a350a-b918-4d35-8cf0-99c79af95269	493.9	59.7
b1ef6021-c779-4373-9147-6f6e442bbdab	5113.9	828.8
8b9a93ea-63f0-4d01-8caa-2bbd3f0653e3	69.6	455
a66fb30f-7d80-4b27-90c6-4f6b0723ab89	317.5	532.9
5d224df6-262a-41b2-adb0-530f1909e896	64.7	266.7
3bc70777-1f75-479f-9eb9-7b639881ba19	1118.2	142.5
36a29ed3-81c7-465b-a7f5-5e44a6a99cbf	1723.3	321.5
fff6c9d5-5828-4c71-8bb7-5c462ff5cf9e	690.7	2437.1
\.


--
-- Data for Name: opportunity; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.opportunity (id, employment, higher_education, k12_education, score, technical_vocational_college) FROM stdin;
fc8f1d05-55e8-42cf-967e-002056ba050e	25.33	0	34.77	26.29	0
f2058cdd-dea8-491b-80cf-11d08caceac8	24.88	0	29.04	26.96	0
da7327c5-34c9-44bc-bee4-31b6ad5bf1d2	23.75	0	8.12	13.94	0
aa8cf39e-93d7-4be0-99a4-424a4c7613b6	14.41	9.94	30.96	19.85	0
23fe492a-cef7-4e3b-997f-3116bd53c7aa	8.23	0	28.66	18.45	0
8fd7642f-4a31-4a13-95c8-f1ea1aa263ba	21.19	19.66	29.14	22.07	5.25
30597289-57c0-4f26-acc8-aa88b3859d0e	27.46	0	20.3	20.9	0
9b3f28d6-ecf0-4046-abe3-3657d6ea30ae	31.93	30.29	51.57	37.36	20.79
98d8327f-f0be-4486-93fd-fb238582889c	26	0	32.27	20.39	0
723d90e9-a5e3-44cc-9957-f480f928cf02	21.87	14.62	43.74	28.25	23.61
61d735fa-49d1-4b66-87d8-4a7e765aaf8b	0	68.49	73.18	69.46	58.36
ec300bdb-6d8f-4ff4-b3a6-ce6d3c2fab54	0	0	58.57	52.3	30.37
d65a350a-b918-4d35-8cf0-99c79af95269	0	43.26	82.87	67.89	64.75
b1ef6021-c779-4373-9147-6f6e442bbdab	0	53.97	79	69.98	70.42
8b9a93ea-63f0-4d01-8caa-2bbd3f0653e3	0	21.12	54.97	38.46	15.37
a66fb30f-7d80-4b27-90c6-4f6b0723ab89	0	49.95	55.22	52.74	49.61
5d224df6-262a-41b2-adb0-530f1909e896	0	0	37.86	23.31	16.57
3bc70777-1f75-479f-9eb9-7b639881ba19	0	18.88	60.97	42.83	27.27
36a29ed3-81c7-465b-a7f5-5e44a6a99cbf	0	82.47	87.88	86.07	86.92
fff6c9d5-5828-4c71-8bb7-5c462ff5cf9e	0	12.38	59.82	39.77	21.01
\.


--
-- Data for Name: people; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.people (id, score) FROM stdin;
fc8f1d05-55e8-42cf-967e-002056ba050e	33.35
f2058cdd-dea8-491b-80cf-11d08caceac8	25.61
da7327c5-34c9-44bc-bee4-31b6ad5bf1d2	29.2
aa8cf39e-93d7-4be0-99a4-424a4c7613b6	22.54
23fe492a-cef7-4e3b-997f-3116bd53c7aa	21.48
8fd7642f-4a31-4a13-95c8-f1ea1aa263ba	31.71
30597289-57c0-4f26-acc8-aa88b3859d0e	26.04
9b3f28d6-ecf0-4046-abe3-3657d6ea30ae	38.39
98d8327f-f0be-4486-93fd-fb238582889c	29.22
723d90e9-a5e3-44cc-9957-f480f928cf02	26.67
61d735fa-49d1-4b66-87d8-4a7e765aaf8b	67.2
ec300bdb-6d8f-4ff4-b3a6-ce6d3c2fab54	43.37
d65a350a-b918-4d35-8cf0-99c79af95269	68.08
b1ef6021-c779-4373-9147-6f6e442bbdab	58.73
8b9a93ea-63f0-4d01-8caa-2bbd3f0653e3	37.07
a66fb30f-7d80-4b27-90c6-4f6b0723ab89	53.24
5d224df6-262a-41b2-adb0-530f1909e896	17.35
3bc70777-1f75-479f-9eb9-7b639881ba19	57.33
36a29ed3-81c7-465b-a7f5-5e44a6a99cbf	85.56
fff6c9d5-5828-4c71-8bb7-5c462ff5cf9e	30.86
\.


--
-- Data for Name: recreation; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.recreation (id, community_centers, parks, recreation_trails, score) FROM stdin;
fc8f1d05-55e8-42cf-967e-002056ba050e	0	60.53	27.24	44.99
f2058cdd-dea8-491b-80cf-11d08caceac8	2.82	42.77	5.18	19.63
da7327c5-34c9-44bc-bee4-31b6ad5bf1d2	5.92	19.45	0	14.25
aa8cf39e-93d7-4be0-99a4-424a4c7613b6	11.36	59.42	9.3	29.86
23fe492a-cef7-4e3b-997f-3116bd53c7aa	0	51.16	0	51.16
8fd7642f-4a31-4a13-95c8-f1ea1aa263ba	7.19	40.51	27.83	27.74
30597289-57c0-4f26-acc8-aa88b3859d0e	0	64.46	30.33	48.53
9b3f28d6-ecf0-4046-abe3-3657d6ea30ae	39.02	59.59	52.56	51.99
98d8327f-f0be-4486-93fd-fb238582889c	0	50.01	0	50.01
723d90e9-a5e3-44cc-9957-f480f928cf02	11.55	59.14	36.42	39.29
61d735fa-49d1-4b66-87d8-4a7e765aaf8b	48.15	75.06	77.22	69.09
ec300bdb-6d8f-4ff4-b3a6-ce6d3c2fab54	19.83	75.51	23.75	43.47
d65a350a-b918-4d35-8cf0-99c79af95269	80.95	83.46	51.86	71.77
b1ef6021-c779-4373-9147-6f6e442bbdab	42.81	76.6	29.67	51.73
8b9a93ea-63f0-4d01-8caa-2bbd3f0653e3	18.91	72.29	25.93	42.72
a66fb30f-7d80-4b27-90c6-4f6b0723ab89	41.61	72.31	72.08	64.55
5d224df6-262a-41b2-adb0-530f1909e896	24.76	68.57	29.64	43.99
3bc70777-1f75-479f-9eb9-7b639881ba19	15.9	77.12	0	53.57
36a29ed3-81c7-465b-a7f5-5e44a6a99cbf	84.07	90.24	97.25	91.15
fff6c9d5-5828-4c71-8bb7-5c462ff5cf9e	25.95	70.34	35.26	46.96
\.


--
-- Data for Name: retail; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.retail (id, score) FROM stdin;
fc8f1d05-55e8-42cf-967e-002056ba050e	37.81
f2058cdd-dea8-491b-80cf-11d08caceac8	44.73
da7327c5-34c9-44bc-bee4-31b6ad5bf1d2	0
aa8cf39e-93d7-4be0-99a4-424a4c7613b6	24.02
23fe492a-cef7-4e3b-997f-3116bd53c7aa	3.11
8fd7642f-4a31-4a13-95c8-f1ea1aa263ba	15.31
30597289-57c0-4f26-acc8-aa88b3859d0e	0
9b3f28d6-ecf0-4046-abe3-3657d6ea30ae	36.35
98d8327f-f0be-4486-93fd-fb238582889c	32.38
723d90e9-a5e3-44cc-9957-f480f928cf02	32.9
61d735fa-49d1-4b66-87d8-4a7e765aaf8b	64.91
ec300bdb-6d8f-4ff4-b3a6-ce6d3c2fab54	55.83
d65a350a-b918-4d35-8cf0-99c79af95269	82.11
b1ef6021-c779-4373-9147-6f6e442bbdab	32.72
8b9a93ea-63f0-4d01-8caa-2bbd3f0653e3	35.69
a66fb30f-7d80-4b27-90c6-4f6b0723ab89	63.44
5d224df6-262a-41b2-adb0-530f1909e896	23.68
3bc70777-1f75-479f-9eb9-7b639881ba19	41.8
36a29ed3-81c7-465b-a7f5-5e44a6a99cbf	82.21
fff6c9d5-5828-4c71-8bb7-5c462ff5cf9e	65.13
\.


--
-- Data for Name: seaql_migrations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.seaql_migrations (version, applied_at) FROM stdin;
m20220101_000001_main	1736890164
m20231010_232527_city_submission	1736890164
m20240202_004130_brokenspoke_analyzer_pipeline	1736890164
\.


--
-- Data for Name: speed_limit; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.speed_limit (id, city_id, created_at, residential) FROM stdin;
1	58f1f419-ac98-4f12-9f4d-d78939773041	2024-03-08 15:30:00+00	25
2	6ba8c886-419a-4880-a749-b7e4246bb78c	2024-03-05 17:23:00+00	25
3	06b9181f-f879-40d5-b531-c239caa8ccd9	2023-08-23 13:59:00+00	35
4	09c049b6-213b-405c-bc4e-178346ff814d	2021-02-25 13:26:00+00	25
5	06118205-6e16-46da-bc9c-459860189e8f	2024-03-20 17:11:00+00	25
6	4ccfcf6b-17c9-4006-b1cd-7e2829d85d54	2023-12-06 08:32:00+00	25
7	4febf356-c079-41c9-81b4-e930dcf2daac	2017-04-29 06:59:00+00	25
8	712846c3-d432-480b-b582-d47730cf90bf	2017-04-29 06:58:00+00	25
9	3fa975ec-55af-4a63-addf-a36b920fc9a7	2024-03-14 09:22:00+00	25
10	fbdeaa5e-92d7-43b7-b00b-915b8c10c9a7	2022-02-01 19:14:00+00	25
11	a1fe143b-6253-40d8-bfb0-7bfaeccee6c4	2023-04-17 16:59:00+00	20
12	6ef16bd9-0759-447d-b7af-744888b824ca	2024-04-04 13:13:00+00	30
13	a6c14f18-ff74-42fb-9324-72f2c8d0fb66	2024-04-01 15:00:00+00	20
14	bbd47ea1-3ca9-41b6-803d-ab7c9505e768	2023-01-07 21:28:00+00	25
15	daffa2db-4980-4ddd-8bf2-4bf79ef09e10	2024-04-03 16:47:00+00	30
16	f249b76f-3db0-4641-b2d7-b65aa69ee229	2023-04-12 16:18:00+00	31
17	8f11086a-b44a-44cf-a755-1fc9c257a48d	2021-03-24 12:30:00+00	31
18	bd1f74b3-bcc8-44c2-959f-f959accc3712	2024-04-12 10:49:00+00	25
19	3bd1329c-4f97-4278-beae-c025a6a1ea66	2023-04-18 09:34:00+00	20
20	bfdde387-2429-490a-95f0-4e719ef7aa78	2022-03-28 16:00:00+00	31
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

COPY public.summary (id, city_id, created_at, score, version) FROM stdin;
fc8f1d05-55e8-42cf-967e-002056ba050e	58f1f419-ac98-4f12-9f4d-d78939773041	2024-03-08 15:30:00+00	26	24.01
f2058cdd-dea8-491b-80cf-11d08caceac8	6ba8c886-419a-4880-a749-b7e4246bb78c	2024-03-05 17:23:00+00	26	24.01
da7327c5-34c9-44bc-bee4-31b6ad5bf1d2	06b9181f-f879-40d5-b531-c239caa8ccd9	2023-08-23 13:59:00+00	15	23.01
aa8cf39e-93d7-4be0-99a4-424a4c7613b6	09c049b6-213b-405c-bc4e-178346ff814d	2021-02-25 13:26:00+00	20	21.01
23fe492a-cef7-4e3b-997f-3116bd53c7aa	06118205-6e16-46da-bc9c-459860189e8f	2024-03-20 17:11:00+00	24	24.01
8fd7642f-4a31-4a13-95c8-f1ea1aa263ba	4ccfcf6b-17c9-4006-b1cd-7e2829d85d54	2023-12-06 08:32:00+00	20	23.03
30597289-57c0-4f26-acc8-aa88b3859d0e	4febf356-c079-41c9-81b4-e930dcf2daac	2017-04-29 06:59:00+00	22	17.01
9b3f28d6-ecf0-4046-abe3-3657d6ea30ae	712846c3-d432-480b-b582-d47730cf90bf	2017-04-29 06:58:00+00	37	17.01
98d8327f-f0be-4486-93fd-fb238582889c	3fa975ec-55af-4a63-addf-a36b920fc9a7	2024-03-14 09:22:00+00	26	24.01
723d90e9-a5e3-44cc-9957-f480f928cf02	fbdeaa5e-92d7-43b7-b00b-915b8c10c9a7	2022-02-01 19:14:00+00	26	22.01
61d735fa-49d1-4b66-87d8-4a7e765aaf8b	a1fe143b-6253-40d8-bfb0-7bfaeccee6c4	2023-04-17 16:59:00+00	67	23.01
ec300bdb-6d8f-4ff4-b3a6-ce6d3c2fab54	6ef16bd9-0759-447d-b7af-744888b824ca	2024-04-04 13:13:00+00	44	24.01
d65a350a-b918-4d35-8cf0-99c79af95269	a6c14f18-ff74-42fb-9324-72f2c8d0fb66	2024-04-01 15:00:00+00	75	24.01
b1ef6021-c779-4373-9147-6f6e442bbdab	bbd47ea1-3ca9-41b6-803d-ab7c9505e768	2023-01-07 21:28:00+00	55	23.01
8b9a93ea-63f0-4d01-8caa-2bbd3f0653e3	daffa2db-4980-4ddd-8bf2-4bf79ef09e10	2024-04-03 16:47:00+00	32	24.01
a66fb30f-7d80-4b27-90c6-4f6b0723ab89	f249b76f-3db0-4641-b2d7-b65aa69ee229	2023-04-12 16:18:00+00	55	23.01
5d224df6-262a-41b2-adb0-530f1909e896	8f11086a-b44a-44cf-a755-1fc9c257a48d	2021-03-24 12:30:00+00	24	21.01
3bc70777-1f75-479f-9eb9-7b639881ba19	bd1f74b3-bcc8-44c2-959f-f959accc3712	2024-04-12 10:49:00+00	47	24.01
36a29ed3-81c7-465b-a7f5-5e44a6a99cbf	3bd1329c-4f97-4278-beae-c025a6a1ea66	2023-04-18 09:34:00+00	85	23.01
fff6c9d5-5828-4c71-8bb7-5c462ff5cf9e	bfdde387-2429-490a-95f0-4e719ef7aa78	2022-03-28 16:00:00+00	43	22.01
\.


--
-- Data for Name: transit; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.transit (id, score) FROM stdin;
fc8f1d05-55e8-42cf-967e-002056ba050e	0
f2058cdd-dea8-491b-80cf-11d08caceac8	0
da7327c5-34c9-44bc-bee4-31b6ad5bf1d2	0
aa8cf39e-93d7-4be0-99a4-424a4c7613b6	18.08
23fe492a-cef7-4e3b-997f-3116bd53c7aa	0
8fd7642f-4a31-4a13-95c8-f1ea1aa263ba	11.71
30597289-57c0-4f26-acc8-aa88b3859d0e	9.49
9b3f28d6-ecf0-4046-abe3-3657d6ea30ae	0
98d8327f-f0be-4486-93fd-fb238582889c	0.41
723d90e9-a5e3-44cc-9957-f480f928cf02	6.51
61d735fa-49d1-4b66-87d8-4a7e765aaf8b	59.4
ec300bdb-6d8f-4ff4-b3a6-ce6d3c2fab54	27.47
d65a350a-b918-4d35-8cf0-99c79af95269	76.04
b1ef6021-c779-4373-9147-6f6e442bbdab	52.79
8b9a93ea-63f0-4d01-8caa-2bbd3f0653e3	14.19
a66fb30f-7d80-4b27-90c6-4f6b0723ab89	43.07
5d224df6-262a-41b2-adb0-530f1909e896	12.71
3bc70777-1f75-479f-9eb9-7b639881ba19	32.39
36a29ed3-81c7-465b-a7f5-5e44a6a99cbf	85.06
fff6c9d5-5828-4c71-8bb7-5c462ff5cf9e	30.01
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
-- Name: census_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.census_id_seq', 20, true);


--
-- Name: fargate_price_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.fargate_price_id_seq', 1, true);


--
-- Name: speed_limit_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.speed_limit_id_seq', 20, true);


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
-- Name: census census_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.census
    ADD CONSTRAINT census_pkey PRIMARY KEY (id);


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
-- Name: speed_limit speed_limit_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.speed_limit
    ADD CONSTRAINT speed_limit_pkey PRIMARY KEY (id);


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
-- Name: census_city_id_created_at_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX census_city_id_created_at_idx ON public.census USING btree (city_id, created_at DESC);


--
-- Name: census_city_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX census_city_id_idx ON public.census USING btree (city_id);


--
-- Name: census_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX census_id_idx ON public.census USING btree (id);


--
-- Name: city_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX city_id_idx ON public.city USING btree (id);


--
-- Name: speed_limit_city_id_created_at_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX speed_limit_city_id_created_at_idx ON public.speed_limit USING btree (city_id, created_at DESC);


--
-- Name: speed_limit_city_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX speed_limit_city_id_idx ON public.speed_limit USING btree (city_id);


--
-- Name: speed_limit_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX speed_limit_id_idx ON public.speed_limit USING btree (id);


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
-- Name: census census_city_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.census
    ADD CONSTRAINT census_city_id_fkey FOREIGN KEY (city_id) REFERENCES public.city(id) ON DELETE CASCADE;


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
-- Name: speed_limit speed_limit_city_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.speed_limit
    ADD CONSTRAINT speed_limit_city_id_fkey FOREIGN KEY (city_id) REFERENCES public.city(id) ON DELETE CASCADE;


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

