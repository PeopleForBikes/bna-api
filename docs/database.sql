--
-- PostgreSQL database dump
--

-- Dumped from database version 15.2 (Debian 15.2-1.pgdg110+1)
-- Dumped by pg_dump version 15.7 (Homebrew)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: approval_status; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.approval_status AS ENUM (
    'Pending',
    'Approved',
    'Rejected'
);


ALTER TYPE public.approval_status OWNER TO postgres;

--
-- Name: bna_region; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.bna_region AS ENUM (
    'Mid-Atlantic',
    'Midwest',
    'Mountain',
    'New England',
    'Pacific',
    'South'
);


ALTER TYPE public.bna_region OWNER TO postgres;

--
-- Name: brokenspoke_status; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.brokenspoke_status AS ENUM (
    'pending',
    'started',
    'complete'
);


ALTER TYPE public.brokenspoke_status OWNER TO postgres;

--
-- Name: brokenspoke_step; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.brokenspoke_step AS ENUM (
    'sqs_message',
    'setup',
    'analysis',
    'cleanup'
);


ALTER TYPE public.brokenspoke_step OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: brokenspoke_pipeline; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.brokenspoke_pipeline (
    state_machine_id uuid NOT NULL,
    step public.brokenspoke_step,
    sqs_message json,
    fargate_task_arn character varying,
    s3_bucket character varying,
    start_time timestamp with time zone NOT NULL,
    end_time timestamp with time zone,
    torn_down boolean,
    results_posted boolean,
    cost numeric
);


ALTER TABLE public.brokenspoke_pipeline OWNER TO postgres;

--
-- Name: census; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.census (
    census_id integer NOT NULL,
    city_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    fips_code character varying NOT NULL,
    pop_size integer NOT NULL,
    population integer NOT NULL
);


ALTER TABLE public.census OWNER TO postgres;

--
-- Name: census_census_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.census_census_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.census_census_id_seq OWNER TO postgres;

--
-- Name: census_census_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.census_census_id_seq OWNED BY public.census.census_id;


--
-- Name: city; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.city (
    city_id uuid NOT NULL,
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
    bna_uuid uuid NOT NULL,
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
    country_id integer NOT NULL,
    name character varying NOT NULL
);


ALTER TABLE public.country OWNER TO postgres;

--
-- Name: country_country_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.country_country_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.country_country_id_seq OWNER TO postgres;

--
-- Name: country_country_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.country_country_id_seq OWNED BY public.country.country_id;


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


ALTER TABLE public.fargate_price_id_seq OWNER TO postgres;

--
-- Name: fargate_price_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.fargate_price_id_seq OWNED BY public.fargate_price.id;


--
-- Name: features; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.features (
    bna_uuid uuid NOT NULL,
    people double precision,
    retail double precision,
    transit double precision
);


ALTER TABLE public.features OWNER TO postgres;

--
-- Name: infrastructure; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.infrastructure (
    bna_uuid uuid NOT NULL,
    low_stress_miles double precision,
    high_stress_miles double precision
);


ALTER TABLE public.infrastructure OWNER TO postgres;

--
-- Name: opportunity; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.opportunity (
    bna_uuid uuid NOT NULL,
    employment double precision,
    higher_education double precision,
    k12_education double precision,
    score double precision,
    technical_vocational_college double precision
);


ALTER TABLE public.opportunity OWNER TO postgres;

--
-- Name: recreation; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.recreation (
    bna_uuid uuid NOT NULL,
    community_centers double precision,
    parks double precision,
    recreation_trails double precision,
    score double precision
);


ALTER TABLE public.recreation OWNER TO postgres;

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
    speed_limit_id integer NOT NULL,
    city_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    residential integer NOT NULL
);


ALTER TABLE public.speed_limit OWNER TO postgres;

--
-- Name: speed_limit_speed_limit_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.speed_limit_speed_limit_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.speed_limit_speed_limit_id_seq OWNER TO postgres;

--
-- Name: speed_limit_speed_limit_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.speed_limit_speed_limit_id_seq OWNED BY public.speed_limit.speed_limit_id;


--
-- Name: state_region_crosswalk; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.state_region_crosswalk (
    state character varying NOT NULL,
    region public.bna_region NOT NULL
);


ALTER TABLE public.state_region_crosswalk OWNER TO postgres;

--
-- Name: state_speed_limit; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.state_speed_limit (
    state_abbrev character(2) NOT NULL,
    state_fips_code character(2) NOT NULL,
    speed integer NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone
);


ALTER TABLE public.state_speed_limit OWNER TO postgres;

--
-- Name: submission; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.submission (
    id integer NOT NULL,
    first_name character varying NOT NULL,
    last_name character varying NOT NULL,
    title character varying,
    organization character varying,
    email character varying NOT NULL,
    country character varying NOT NULL,
    city character varying NOT NULL,
    region character varying,
    fips_code character varying DEFAULT '0'::character varying NOT NULL,
    consent boolean NOT NULL,
    status public.approval_status DEFAULT 'Pending'::public.approval_status NOT NULL,
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


ALTER TABLE public.submission_id_seq OWNER TO postgres;

--
-- Name: submission_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.submission_id_seq OWNED BY public.submission.id;


--
-- Name: summary; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.summary (
    bna_uuid uuid NOT NULL,
    city_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    score double precision NOT NULL,
    version character varying NOT NULL
);


ALTER TABLE public.summary OWNER TO postgres;

--
-- Name: census census_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.census ALTER COLUMN census_id SET DEFAULT nextval('public.census_census_id_seq'::regclass);


--
-- Name: country country_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.country ALTER COLUMN country_id SET DEFAULT nextval('public.country_country_id_seq'::regclass);


--
-- Name: fargate_price id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.fargate_price ALTER COLUMN id SET DEFAULT nextval('public.fargate_price_id_seq'::regclass);


--
-- Name: speed_limit speed_limit_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.speed_limit ALTER COLUMN speed_limit_id SET DEFAULT nextval('public.speed_limit_speed_limit_id_seq'::regclass);


--
-- Name: submission id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.submission ALTER COLUMN id SET DEFAULT nextval('public.submission_id_seq'::regclass);


--
-- Name: brokenspoke_pipeline brokenspoke_pipeline_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.brokenspoke_pipeline
    ADD CONSTRAINT brokenspoke_pipeline_pkey PRIMARY KEY (state_machine_id);


--
-- Name: census census_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.census
    ADD CONSTRAINT census_pkey PRIMARY KEY (census_id);


--
-- Name: city city_city_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.city
    ADD CONSTRAINT city_city_id_key UNIQUE (city_id);


--
-- Name: city city_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.city
    ADD CONSTRAINT city_pkey PRIMARY KEY (country, state, name);


--
-- Name: core_services core_services_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.core_services
    ADD CONSTRAINT core_services_pkey PRIMARY KEY (bna_uuid);


--
-- Name: country country_name_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.country
    ADD CONSTRAINT country_name_key UNIQUE (name);


--
-- Name: country country_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.country
    ADD CONSTRAINT country_pkey PRIMARY KEY (country_id);


--
-- Name: fargate_price fargate_price_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.fargate_price
    ADD CONSTRAINT fargate_price_pkey PRIMARY KEY (id);


--
-- Name: features features_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.features
    ADD CONSTRAINT features_pkey PRIMARY KEY (bna_uuid);


--
-- Name: infrastructure infrastructure_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.infrastructure
    ADD CONSTRAINT infrastructure_pkey PRIMARY KEY (bna_uuid);


--
-- Name: opportunity opportunity_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.opportunity
    ADD CONSTRAINT opportunity_pkey PRIMARY KEY (bna_uuid);


--
-- Name: recreation recreation_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.recreation
    ADD CONSTRAINT recreation_pkey PRIMARY KEY (bna_uuid);


--
-- Name: seaql_migrations seaql_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.seaql_migrations
    ADD CONSTRAINT seaql_migrations_pkey PRIMARY KEY (version);


--
-- Name: speed_limit speed_limit_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.speed_limit
    ADD CONSTRAINT speed_limit_pkey PRIMARY KEY (speed_limit_id);


--
-- Name: state_region_crosswalk state_region_crosswalk_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.state_region_crosswalk
    ADD CONSTRAINT state_region_crosswalk_pkey PRIMARY KEY (state);


--
-- Name: state_speed_limit state_speed_limit_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.state_speed_limit
    ADD CONSTRAINT state_speed_limit_pkey PRIMARY KEY (state_abbrev);


--
-- Name: submission submission_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.submission
    ADD CONSTRAINT submission_pkey PRIMARY KEY (id);


--
-- Name: summary summary_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.summary
    ADD CONSTRAINT summary_pkey PRIMARY KEY (bna_uuid);


--
-- Name: census_census_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX census_census_id_idx ON public.census USING btree (census_id);


--
-- Name: census_city_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX census_city_id_idx ON public.census USING btree (city_id);


--
-- Name: city_city_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX city_city_id_idx ON public.city USING btree (city_id);


--
-- Name: speed_limit_city_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX speed_limit_city_id_idx ON public.speed_limit USING btree (city_id);


--
-- Name: speed_limit_speed_limit_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX speed_limit_speed_limit_id_idx ON public.speed_limit USING btree (speed_limit_id);


--
-- Name: census census_city_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.census
    ADD CONSTRAINT census_city_id_fkey FOREIGN KEY (city_id) REFERENCES public.city(city_id) ON DELETE CASCADE;


--
-- Name: core_services core_services_bna_uuid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.core_services
    ADD CONSTRAINT core_services_bna_uuid_fkey FOREIGN KEY (bna_uuid) REFERENCES public.summary(bna_uuid) ON DELETE CASCADE;


--
-- Name: features features_bna_uuid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.features
    ADD CONSTRAINT features_bna_uuid_fkey FOREIGN KEY (bna_uuid) REFERENCES public.summary(bna_uuid) ON DELETE CASCADE;


--
-- Name: infrastructure infrastructure_bna_uuid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.infrastructure
    ADD CONSTRAINT infrastructure_bna_uuid_fkey FOREIGN KEY (bna_uuid) REFERENCES public.summary(bna_uuid) ON DELETE CASCADE;


--
-- Name: opportunity opportunity_bna_uuid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.opportunity
    ADD CONSTRAINT opportunity_bna_uuid_fkey FOREIGN KEY (bna_uuid) REFERENCES public.summary(bna_uuid) ON DELETE CASCADE;


--
-- Name: recreation recreation_bna_uuid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.recreation
    ADD CONSTRAINT recreation_bna_uuid_fkey FOREIGN KEY (bna_uuid) REFERENCES public.summary(bna_uuid) ON DELETE CASCADE;


--
-- Name: speed_limit speed_limit_city_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.speed_limit
    ADD CONSTRAINT speed_limit_city_id_fkey FOREIGN KEY (city_id) REFERENCES public.city(city_id) ON DELETE CASCADE;


--
-- Name: summary summary_city_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.summary
    ADD CONSTRAINT summary_city_id_fkey FOREIGN KEY (city_id) REFERENCES public.city(city_id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

