CREATE TABLE IF NOT EXISTS `customers` (
    `id` VARCHAR(255) PRIMARY KEY,
    `email` VARCHAR(255) NOT NULL,
    `stripe_customer_id` VARCHAR(255) NOT NULL,
    `name` VARCHAR(255),
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE KEY `uk_stripe_customer_id` (`stripe_customer_id`),
    UNIQUE KEY `uk_email` (`email`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
