CREATE TABLE public."user" (
	"id" serial NOT NULL,
	"username" varchar(64) NOT NULL,
	"password_hash" varchar(128) NOT NULL,
	"name" varchar(100) NOT NULL,
	"email" varchar(100) NOT NULL,
	"username_lower" varchar(64) NOT NULL,
	"email_lower" varchar(100) NOT NULL,
	"website" text NULL,
	"avatar_url" text NULL,
	"permission" int4 NOT NULL DEFAULT 0,
	"created_at" timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"modified_at" timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"last_login_time" timestamptz NOT NULL DEFAULT '1970-01-01 08:00:00+00'::timestamp with time zone,
	"status" int4 NOT NULL DEFAULT 0,
	CONSTRAINT "pk_user" PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "idx_user__email_lower" ON public."user" USING btree ("email_lower");
CREATE UNIQUE INDEX "idx_user__username_lower" ON public."user" USING btree ("username_lower");

CREATE TABLE public."category" (
	"id" serial NOT NULL,
	"slug" varchar(100) NOT NULL,
	"name" varchar(200) NOT NULL,
	"description" text NULL,
	"order" int4 NOT NULL DEFAULT 0,
	"parent" int4 NULL,
	CONSTRAINT "pk_category" PRIMARY KEY ("id"),
	CONSTRAINT "fk_category__parent" FOREIGN KEY ("parent") REFERENCES "category"("id")
);
CREATE INDEX "idx_category__name" ON public."category" USING btree ("name");
CREATE INDEX "idx_category__parent" ON public."category" USING btree ("parent");
CREATE UNIQUE INDEX "idx_category__slug" ON public."category" USING btree ("slug");

CREATE TABLE public."tag" (
	"id" serial NOT NULL,
	"name" varchar(200) NOT NULL,
	CONSTRAINT "pk_tag" PRIMARY KEY ("id"),
	CONSTRAINT "uk_tag__name" UNIQUE ("name")
);
CREATE INDEX "idx_tag__name" ON public."tag" USING btree ("name");

CREATE TABLE public."content" (
	"id" serial NOT NULL,
	"user" int4 NULL,
	"created_at" timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"modified_at" timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"time" timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"title" varchar(500) NULL,
	"content" text NOT NULL,
	"draft_content" text NULL,
	"slug" varchar(233) NULL,
	"category" int4 NULL,
	"order_level" int4 NOT NULL DEFAULT 0,
	"type" int4 NOT NULL,
	"status" int4 NOT NULL DEFAULT 0,
	"view_password" varchar(233) NULL,
	"allow_comment" bool NOT NULL DEFAULT true,
	"allow_feed" bool NOT NULL DEFAULT true,
	"parent" int4 NULL,
	CONSTRAINT "pk_content" PRIMARY KEY ("id"),
	CONSTRAINT "fk_content__category" FOREIGN KEY ("category") REFERENCES "category"("id"),
	CONSTRAINT "fk_content__parent" FOREIGN KEY ("parent") REFERENCES "content"("id"),
	CONSTRAINT "fk_content__user" FOREIGN KEY ("user") REFERENCES "user"("id")
);
CREATE INDEX "idx_content__category" ON public."content" USING btree ("category");
CREATE INDEX "idx_content__order_level" ON public."content" USING btree ("order_level");
CREATE INDEX "idx_content__user" ON public."content" USING btree ("user");

CREATE TABLE public."assoc_tag_content" (
	"id" serial NOT NULL,
	"tag" int4 NOT NULL,
	"content" int4 NOT NULL,
	CONSTRAINT "pk_assoc_tag_content" PRIMARY KEY ("id"),
	CONSTRAINT "uk_assoc_tag_content" UNIQUE ("tag", "content"),
	CONSTRAINT "fk_assoc_tag_content__content" FOREIGN KEY ("content") REFERENCES "content"("id") ON DELETE CASCADE,
	CONSTRAINT "fk_assoc_tag_content__tag" FOREIGN KEY ("tag") REFERENCES "tag"("id") ON DELETE CASCADE
);
CREATE INDEX "idx_assoc_tag_content__content" ON public."assoc_tag_content" USING btree ("content");
CREATE INDEX "idx_assoc_tag_content__tag" ON public."assoc_tag_content" USING btree ("tag");

CREATE TABLE public."file" (
	"id" serial NOT NULL,
	"key" varchar(500) NOT NULL,
	"filename" text NOT NULL,
	"content" int4 NULL,
	"user" int4 NOT NULL,
	"time" timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT "pk_file" PRIMARY KEY ("id"),
	CONSTRAINT "uk_file__key" UNIQUE ("key"),
	CONSTRAINT "fk_file__content" FOREIGN KEY ("content") REFERENCES "content"("id") ON DELETE SET NULL,
	CONSTRAINT "fk_file__user" FOREIGN KEY ("user") REFERENCES "user"("id")
);
CREATE INDEX "idx_file__content" ON public."file" USING btree ("content");
CREATE INDEX "idx_file__key" ON public."file" USING btree ("key");
CREATE INDEX "idx_file__user" ON public."file" USING btree ("user");
COMMENT ON COLUMN public."file"."key" IS 'File Key, can be URL or something like `{upload_dir}/file.key`';
COMMENT ON COLUMN public."file"."filename" IS 'Original file name';
COMMENT ON COLUMN public."file"."content" IS 'Linked content ID';

CREATE TABLE public."comment" (
	"id" serial NOT NULL,
	"user" int4 NULL,
	"author_name" varchar(200) NOT NULL,
	"author_mail" varchar(200) NULL,
	"author_link" varchar(200) NULL,
	"ip" inet NULL,
	"time" timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"user_agent" text NULL,
	"text" text NOT NULL,
	"status" int4 NOT NULL DEFAULT 0,
	"reply_to" int4 NULL, -- This indicates which comment is this replys to.
	"parent" int4 NULL, -- Only ONE or NO parent-child relation is allowed.
	"content" int4 NOT NULL,
	CONSTRAINT "pk_comment" PRIMARY KEY ("id"),
	CONSTRAINT "fk_comment__content" FOREIGN KEY ("content") REFERENCES "content"("id"),
	CONSTRAINT "fk_comment__parent" FOREIGN KEY ("parent") REFERENCES "comment"("id") ON DELETE CASCADE,
	CONSTRAINT "fk_comment__reply_to" FOREIGN KEY ("reply_to") REFERENCES "comment"("id") ON DELETE SET NULL,
	CONSTRAINT "fk_comment__user" FOREIGN KEY ("user") REFERENCES "user"("id") ON DELETE SET NULL
);
CREATE INDEX "idx_comment__content" ON public."comment" USING btree ("content");
CREATE INDEX "idx_comment__parent" ON public."comment" USING btree ("parent");
CREATE INDEX "idx_comment__reply_to" ON public."comment" USING btree ("reply_to");
CREATE INDEX "idx_comment__user" ON public."comment" USING btree ("user");
COMMENT ON COLUMN public."comment"."reply_to" IS 'This indicates which comment is this replys to.';
COMMENT ON COLUMN public."comment"."parent" IS 'Only ONE or NO parent-child relation is allowed.';
