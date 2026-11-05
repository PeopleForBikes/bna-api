-- Fixtures for the Hurl suite.
--
-- Loaded after the migrations and baseline.sql. Adds only what read-only tests
-- need and the baseline lacks. Keep fixed identifiers and timestamps so the
-- generated dump stays reproducible.

-- Fargate prices: pin the price inserted by the migrations and add two more,
-- so sorting and `latest` can be tested.
UPDATE public.fargate_price
SET created_at = '2024-01-01 00:00:00+00'
WHERE id = 1;

INSERT INTO public.fargate_price (id, per_second, created_at) VALUES
(2, 0.0000405, '2025-01-01 00:00:00+00'),
(3, 0.0000412, '2026-01-01 00:00:00+00');

SELECT pg_catalog.setval('public.fargate_price_id_seq', 3, true);

-- A city in a country without regions: the region and the state are the
-- country name.
INSERT INTO public.city (
    id, country, state, name, latitude, longitude, region, state_abbrev,
    residential_speed_limit, created_at, updated_at, fips_code
) VALUES (
    'f0000000-0000-4000-8000-00000000c001', 'Netherlands', 'Netherlands',
    'Utrecht', 52.0907, 5.1214, 'Netherlands', NULL, NULL,
    '2024-06-01 00:00:00+00', NULL, '0'
);

-- Ratings: one for Utrecht, and three more in 2019 so that 2019 has 5 rated
-- cities (Chattanooga and Columbia already have one).
INSERT INTO public.summary (
    id, city_id, created_at, pop_size, population,
    residential_speed_limit_override, score, version
) VALUES
(
    'f0000000-0000-4000-8000-00000000a001',
    'f0000000-0000-4000-8000-00000000c001',
    '2024-06-01 00:00:00+00', 2, 361924, NULL, 42, '24.06'
),
(
    'f0000000-0000-4000-8000-00000000a002',
    'a18348b9-fcbd-46de-975c-ee74119519c3', -- Temecula
    '2019-03-01 00:00:00+00', 2, 114761, NULL, 15, '19.01'
),
(
    'f0000000-0000-4000-8000-00000000a003',
    'bf5f3e41-f394-4c0b-9550-060802adc6fa', -- Somerset
    '2019-03-02 00:00:00+00', 0, 6048, NULL, 28, '19.01'
),
(
    'f0000000-0000-4000-8000-00000000a004',
    '8fbe8758-1dc7-4da1-8926-34a6baf5858a', -- Waipahu
    '2019-03-03 00:00:00+00', 1, 43485, NULL, 9, '19.01'
);

INSERT INTO public.core_services (
    id, dentists, doctors, grocery, hospitals, pharmacies, score,
    social_services
) VALUES
('f0000000-0000-4000-8000-00000000a001', 40, 45, 60, 35, 50, 46, 30),
('f0000000-0000-4000-8000-00000000a002', 5, 8, 12, 0, 6, 7, 0),
('f0000000-0000-4000-8000-00000000a003', 20, 25, 30, 10, 22, 24, 5),
('f0000000-0000-4000-8000-00000000a004', 2, 3, 6, 0, 4, 3, 0);

INSERT INTO public.infrastructure (id, low_stress_miles, high_stress_miles)
VALUES
('f0000000-0000-4000-8000-00000000a001', 310.4, 120.2),
('f0000000-0000-4000-8000-00000000a002', 40.1, 290.7),
('f0000000-0000-4000-8000-00000000a003', 12.5, 31.0),
('f0000000-0000-4000-8000-00000000a004', 8.2, 64.9);

INSERT INTO public.opportunity (
    id, employment, higher_education, k12_education, score,
    technical_vocational_college
) VALUES
('f0000000-0000-4000-8000-00000000a001', 44, 38, 50, 45, 30),
('f0000000-0000-4000-8000-00000000a002', 14, 0, 18, 13, 0),
('f0000000-0000-4000-8000-00000000a003', 30, 0, 35, 31, 0),
('f0000000-0000-4000-8000-00000000a004', 10, 0, 12, 9, 0);

INSERT INTO public.people (id, score) VALUES
('f0000000-0000-4000-8000-00000000a001', 41),
('f0000000-0000-4000-8000-00000000a002', 16),
('f0000000-0000-4000-8000-00000000a003', 27),
('f0000000-0000-4000-8000-00000000a004', 10);

INSERT INTO public.recreation (
    id, community_centers, parks, recreation_trails, score
) VALUES
('f0000000-0000-4000-8000-00000000a001', 35, 55, 40, 44),
('f0000000-0000-4000-8000-00000000a002', 0, 20, 10, 15),
('f0000000-0000-4000-8000-00000000a003', 10, 40, 0, 26),
('f0000000-0000-4000-8000-00000000a004', 0, 12, 0, 8);

INSERT INTO public.retail (id, score) VALUES
('f0000000-0000-4000-8000-00000000a001', 47),
('f0000000-0000-4000-8000-00000000a002', 11),
('f0000000-0000-4000-8000-00000000a003', 29),
('f0000000-0000-4000-8000-00000000a004', 7);

INSERT INTO public.transit (id, score) VALUES
('f0000000-0000-4000-8000-00000000a001', 39),
('f0000000-0000-4000-8000-00000000a002', 0),
('f0000000-0000-4000-8000-00000000a003', 0),
('f0000000-0000-4000-8000-00000000a004', 6);

-- Measures on the latest ratings of Chattanooga and Columbia, and on Utrecht.
INSERT INTO public.measure (id, buffered_lane, lane, path, sharrow, track)
VALUES
('5885ec0d-6af5-43c1-9978-d2389e40b95f', 12.3, 45.6, 30.1, 8.4, 2.2),
('666bddf7-5e3b-4def-ad36-960f805877cb', 7.9, 38.2, 52.7, 3.1, 0.0),
('f0000000-0000-4000-8000-00000000a001', 25.0, 80.5, 140.3, 0.0, 35.8);

-- Pipelines: one per status.
INSERT INTO public.bna_pipeline (
    state_machine_id, step, sqs_message, fargate_price_id, fargate_task_arn,
    s3_bucket, status, start_time, end_time, cost
) VALUES
(
    'f0000000-0000-4000-8000-00000000b001', 'Setup',
    '{"city": "Utrecht", "country": "Netherlands", "fips_code": "0", "region": "Netherlands"}',
    NULL, NULL, NULL, 'Pending', '2025-05-01 10:00:00+00', NULL, NULL
),
(
    'f0000000-0000-4000-8000-00000000b002', 'Analysis',
    '{"city": "Columbia", "country": "United States", "fips_code": "2915670", "region": "Missouri"}',
    NULL,
    'arn:aws:ecs:us-west-2:123456789012:task/bna/00000000000000000000000000000b02',
    'united states/missouri/columbia/25.05', 'Processing',
    '2025-05-02 10:00:00+00', NULL, NULL
),
(
    'f0000000-0000-4000-8000-00000000b003', 'Cleanup',
    '{"city": "Chattanooga", "country": "United States", "fips_code": "4714000", "region": "Tennessee"}',
    1,
    'arn:aws:ecs:us-west-2:123456789012:task/bna/00000000000000000000000000000b03',
    'united states/tennessee/chattanooga/25.05', 'Completed',
    '2025-05-03 10:00:00+00', '2025-05-03 14:47:32+00', 0.63749
);

-- Submissions: one per approval status.
INSERT INTO public.submission (
    id, first_name, last_name, occupation, organization, email, country, city,
    region, fips_code, consent, status, created_at
) VALUES
(
    1, 'Jane', 'Doe', 'CTO', 'Organization LLC', 'jane.doe@example.com',
    'United States', 'Santa Rosa', 'New Mexico', '3570670', true, 'Pending',
    '2025-06-01 00:00:00+00'
),
(
    2, 'John', 'Smith', 'Planner', 'City of Leland', 'john.smith@example.com',
    'United States', 'Leland', 'North Carolina', '3737680', true, 'Approved',
    '2025-06-02 00:00:00+00'
),
(
    3, 'Ana', 'Silva', 'Advocate', 'Bike Lisboa', 'ana.silva@example.com',
    'Portugal', 'Lisbon', 'Portugal', '0', true, 'Rejected',
    '2025-06-03 00:00:00+00'
);

SELECT pg_catalog.setval('public.submission_id_seq', 3, true);
