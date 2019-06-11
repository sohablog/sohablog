CREATE TABLE `user` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `username` varchar(64) NOT NULL,
  `password_hash` varchar(128) NOT NULL,
  `name` varchar(100) NOT NULL,
  `email` varchar(100) NOT NULL,
  `username_lower` varchar(64) NOT NULL,
  `email_lower` varchar(100) NOT NULL,
  `website` varchar(200) DEFAULT NULL,
  `avatar_url` text,
  `permission` int(10) unsigned NOT NULL DEFAULT '0',
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `modified_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `last_login_time` datetime NOT NULL DEFAULT '1970-01-01 00:00:00',
  `status` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_username` (`username`),
  UNIQUE KEY `user_email_lower_idx` (`email_lower`) USING BTREE,
  UNIQUE KEY `user_username_lower_idx` (`username_lower`) USING BTREE,
  UNIQUE KEY `uk_email` (`email`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `category` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `slug` varchar(100) NOT NULL,
  `name` varchar(200) NOT NULL,
  `description` text,
  `order` int(11) NOT NULL DEFAULT '0',
  `parent` int(11) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `category_slug_uniq` (`slug`),
  KEY `category_slug_idx` (`slug`) USING BTREE,
  KEY `category_name_idx` (`name`) USING BTREE,
  KEY `category_parent_idx` (`parent`) USING BTREE,
  CONSTRAINT `category_fk` FOREIGN KEY (`parent`) REFERENCES `category` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `tag` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `tag_uniq` (`name`),
  FULLTEXT KEY `tag_name_fulltext_idx` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `content` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `user` int(11) NOT NULL DEFAULT '-1',
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `modified_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `title` varchar(233) DEFAULT NULL,
  `content` longtext NOT NULL,
  `draft_content` longtext DEFAULT NULL,
  `order_level` int(11) NOT NULL DEFAULT '0',
  `type` int(11) NOT NULL,
  `status` int(11) NOT NULL DEFAULT '0',
  `view_password` varchar(233) DEFAULT NULL,
  `allow_comment` tinyint(1) NOT NULL DEFAULT '1',
  `allow_feed` tinyint(1) NOT NULL DEFAULT '1',
  `parent` int(11) DEFAULT NULL,
  `slug` varchar(200) DEFAULT NULL,
  `category` int(11) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `content_user_idx` (`user`) USING BTREE,
  KEY `content_parent_idx` (`parent`) USING BTREE,
  KEY `content_order_level_idx` (`order_level`) USING BTREE,
  KEY `content_type_idx` (`type`) USING BTREE,
  KEY `content_status_idx` (`status`) USING BTREE,
  KEY `content_category_fk` (`category`) USING BTREE,
  FULLTEXT KEY `content_content_idx` (`content`),
  FULLTEXT KEY `content_title_idx` (`title`),
  CONSTRAINT `content_category_fk` FOREIGN KEY (`category`) REFERENCES `category` (`id`),
  CONSTRAINT `content_content_fk` FOREIGN KEY (`parent`) REFERENCES `content` (`id`),
  CONSTRAINT `content_user_fk` FOREIGN KEY (`user`) REFERENCES `user` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `assoc_tag_content` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `tag` int(11) NOT NULL,
  `content` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `assoc_tag_content_tag_uniq_idx` (`tag`,`content`) USING BTREE,
  KEY `assoc_tag_content_tag_idx` (`tag`) USING BTREE,
  KEY `assoc_tag_content_content_idx` (`content`) USING BTREE,
  CONSTRAINT `assoc_content_fk` FOREIGN KEY (`content`) REFERENCES `content` (`id`) ON DELETE CASCADE,
  CONSTRAINT `assoc_tag_fk` FOREIGN KEY (`tag`) REFERENCES `tag` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB;

CREATE TABLE `file` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `key` varchar(500) NOT NULL COMMENT 'File Key, can be URL or something like `{upload_dir}/file.key`',
  `filename` text NOT NULL COMMENT 'Original file name',
  `user` int(11) NOT NULL,
  `content` int(11) DEFAULT NULL COMMENT 'Linked content ID',
  `time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `file_key_uniq` (`key`),
  KEY `file_content_idx` (`content`) USING BTREE,
  KEY `file_user_idx` (`user`) USING BTREE,
  FULLTEXT KEY `file_filename_fulltext_idx` (`filename`),
  CONSTRAINT `file_content_fk` FOREIGN KEY (`content`) REFERENCES `content` (`id`) ON DELETE CASCADE,
  CONSTRAINT `file_user_fk` FOREIGN KEY (`user`) REFERENCES `user` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `comment` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `user` int(11) DEFAULT NULL,
  `author_name` varchar(200) NOT NULL,
  `author_mail` varchar(200) DEFAULT NULL,
  `author_link` varchar(200) DEFAULT NULL,
  `ip` varchar(80) DEFAULT NULL,
  `user_agent` varchar(250) DEFAULT NULL,
  `text` longtext NOT NULL,
  `time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `status` int(11) NOT NULL DEFAULT '0',
  `reply_to` int(11) DEFAULT NULL COMMENT 'This indicates which comment is this replys to.',
  `parent` int(11) DEFAULT NULL COMMENT 'Only ONE or NO parent-child relation is allowed.',
  `content` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `comment_content_fk` (`content`),
  KEY `comment_parent_fk` (`parent`),
  KEY `comment_reply_fk` (`reply_to`),
  KEY `comment_user_fk` (`user`),
  KEY `comment_status_idx` (`status`) USING BTREE,
  CONSTRAINT `comment_content_fk` FOREIGN KEY (`content`) REFERENCES `content` (`id`) ON DELETE CASCADE,
  CONSTRAINT `comment_parent_fk` FOREIGN KEY (`parent`) REFERENCES `comment` (`id`) ON DELETE CASCADE,
  CONSTRAINT `comment_reply_fk` FOREIGN KEY (`reply_to`) REFERENCES `comment` (`id`) ON DELETE SET NULL,
  CONSTRAINT `comment_user_fk` FOREIGN KEY (`user`) REFERENCES `user` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
