--
-- PostgreSQL database dump
--

-- Dumped from database version 15.2 (Debian 15.2-1.pgdg110+1)
-- Dumped by pg_dump version 15.8 (Homebrew)

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
    step character varying,
    sqs_message json,
    fargate_price integer,
    fargate_task_arn character varying,
    s3_bucket character varying,
    status character varying DEFAULT 'Pending'::character varying NOT NULL,
    start_time timestamp with time zone NOT NULL,
    end_time timestamp with time zone,
    torn_down boolean,
    results_posted boolean,
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


ALTER TABLE public.census_id_seq OWNER TO postgres;

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


ALTER TABLE public.fargate_price_id_seq OWNER TO postgres;

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


ALTER TABLE public.speed_limit_id_seq OWNER TO postgres;

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


ALTER TABLE public.submission_id_seq OWNER TO postgres;

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
-- Name: us_state_abbrev_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX us_state_abbrev_idx ON public.us_state USING btree (abbrev);


--
-- Name: us_state_fips_code_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX us_state_fips_code_idx ON public.us_state USING btree (fips_code);


--
-- Name: bna_pipeline bna_pipeline_fargate_price_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bna_pipeline
    ADD CONSTRAINT bna_pipeline_fargate_price_fkey FOREIGN KEY (fargate_price) REFERENCES public.fargate_price(id);


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

