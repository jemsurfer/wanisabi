CREATE TABLE IF NOT EXISTS "resources" (
	"id"	INTEGER NOT NULL UNIQUE,
	"object"	TEXT NOT NULL,
	"url"	TEXT NOT NULL,
	"data_updated_at"	TEXT NOT NULL,
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "level_progressions" (
	"id"	INTEGER NOT NULL UNIQUE,
	"abandoned_at"	TEXT,
	"completed_at"	TEXT,
	"created_at"	TEXT NOT NULL,
	"level"	INTEGER NOT NULL,
	"passed_at"	TEXT,
	"started_at"	TEXT,
	"unlocked_at"	TEXT,
	FOREIGN KEY("id") REFERENCES "resources"("id"),
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "resets" (
	"id"	INTEGER NOT NULL UNIQUE,
	"confirmed_at"	TEXT,
	"created_at"	TEXT NOT NULL,
	"original_level"	INTEGER NOT NULL,
	"target_level"	INTEGER NOT NULL,
	PRIMARY KEY("id"),
	FOREIGN KEY("id") REFERENCES "resources"("id")
);
CREATE TABLE IF NOT EXISTS "spaced_repetition_systems" (
	"id"	INTEGER NOT NULL UNIQUE,
	"burning_stage_position"	INTEGER NOT NULL,
	"created_at"	TEXT NOT NULL,
	"description"	TEXT NOT NULL,
	"name"	TEXT NOT NULL,
	"passing_stage_position"	INTEGER NOT NULL,
	"starting_stage_position"	INTEGER NOT NULL,
	"unlocking_stage_position"	INTEGER NOT NULL,
	"stages"	TEXT NOT NULL,
	FOREIGN KEY("id") REFERENCES "resources"("id"),
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "study_materials" (
	"id"	INTEGER NOT NULL UNIQUE,
	"created_at"	TEXT NOT NULL,
	"hidden"	INTEGER NOT NULL,
	"meaning_note"	TEXT,
	"meaning_synonyms"	TEXT NOT NULL,
	"reading_note"	TEXT,
	"subject_id"	INTEGER NOT NULL,
	"subject_type"	TEXT NOT NULL,
	PRIMARY KEY("id"),
	FOREIGN KEY("id") REFERENCES "resources"("id")
	FOREIGN KEY("subject_id") REFERENCES "subjects"("id")
);
CREATE TABLE IF NOT EXISTS "summary" (
	"id"	INTEGER NOT NULL DEFAULT 0 CHECK("id" = 0),
	"next_reviews_at"	TEXT,
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "users" (
	"id"	TEXT NOT NULL UNIQUE,
	"username"	TEXT NOT NULL,
	"level"	INTEGER NOT NULL,
	"profile_url"	TEXT NOT NULL,
	"started_at"	TEXT NOT NULL,
	"current_vacation_started_at"	TEXT,
	"subscription"	TEXT NOT NULL,
	"preferences"	TEXT NOT NULL,
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "voice_actors" (
	"id"	INTEGER NOT NULL,
	"created_at"	TEXT NOT NULL,
	"name"	TEXT NOT NULL,
	"gender"	TEXT NOT NULL,
	"description"	TEXT NOT NULL,
	FOREIGN KEY("id") REFERENCES "resources"("id"),
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "review_statistics" (
	"id"	INTEGER NOT NULL UNIQUE,
	"created_at"	TEXT NOT NULL,
	"hidden"	INTEGER NOT NULL,
	"meaning_correct"	INTEGER NOT NULL,
	"meaning_current_streak"	INTEGER NOT NULL,
	"meaning_incorrect"	INTEGER NOT NULL,
	"meaning_max_streak"	INTEGER NOT NULL,
	"percentage_correct"	INTEGER NOT NULL,
	"reading_correct"	INTEGER NOT NULL,
	"reading_current_streak"	INTEGER NOT NULL,
	"reading_incorrect"	INTEGER NOT NULL,
	"reading_max_streak"	INTEGER NOT NULL,
	"subject_id"	INTEGER NOT NULL,
	"subject_type"	TEXT NOT NULL,
	PRIMARY KEY("id"),
	FOREIGN KEY("subject_id") REFERENCES "subjects"("id")
);
CREATE TABLE IF NOT EXISTS "reviews" (
	"id"	INTEGER NOT NULL UNIQUE,
	"assignment_id"	INTEGER NOT NULL,
	"created_at"	TEXT NOT NULL,
	"ending_srs_stage"	INTEGER NOT NULL,
	"incorrect_meaning_answers"	INTEGER NOT NULL,
	"incorrect_reading_answers"	INTEGER NOT NULL,
	"spaced_repetition_system_id"	INTEGER NOT NULL,
	"starting_srs_stage"	INTEGER NOT NULL,
	"subject_id"	INTEGER NOT NULL,
	FOREIGN KEY("id") REFERENCES "resources"("id"),
	FOREIGN KEY("assignment_id") REFERENCES "assignments"("id"),
	FOREIGN KEY("subject_id") REFERENCES "subjects"("id"),
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "subjects" (
	"id"	INTEGER NOT NULL UNIQUE,
	"subject_type"	TEXT NOT NULL,
	"created_at"	TEXT NOT NULL,
	"document_url"	TEXT NOT NULL,
	"hidden_at"	TEXT,
	"lesson_position"	INTEGER NOT NULL,
	"level"	INTEGER,
	"meaning_mnemonic"	TEXT NOT NULL,
	"slug"	TEXT NOT NULL,
	"spaced_repetition_system_id"	INTEGER NOT NULL,
	FOREIGN KEY("id") REFERENCES "resources"("id"),
	PRIMARY KEY("id","subject_type"),
	FOREIGN KEY("spaced_repetition_system_id") REFERENCES "spaced_repetition_systems"("id")
);
CREATE TABLE IF NOT EXISTS "vocabulary" (
	"id"	INTEGER NOT NULL UNIQUE,
	"characters"	TEXT NOT NULL,
	"reading_mnemonic"	TEXT NOT NULL,
	"component_subject_ids"	TEXT NOT NULL,
	"context_sentences"	TEXT NOT NULL,
	"meaning_mnemonic"	TEXT NOT NULL,
	"parts_of_speech"	TEXT NOT NULL,
	"pronunciation_audios"	TEXT NOT NULL,
	"readings"	TEXT NOT NULL,
	PRIMARY KEY("id"),
	FOREIGN KEY("id") REFERENCES "subjects"("id")
);
CREATE TABLE IF NOT EXISTS "assignments" (
	"id"	INTEGER NOT NULL UNIQUE,
	"available_at"	TEXT,
	"burned_at"	TEXT,
	"created_at"	TEXT NOT NULL,
	"hidden"	INTEGER NOT NULL,
	"passed_at"	TEXT,
	"resurrected_at"	TEXT,
	"srs_stage"	INTEGER NOT NULL,
	"started_at"	TEXT,
	"subject_id"	INTEGER NOT NULL,
	"subject_type"	TEXT NOT NULL,
	"unlocked_at"	TEXT,
	FOREIGN KEY("subject_id") REFERENCES "subjects"("id"),
	FOREIGN KEY("id") REFERENCES "resources"("id"),
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "kana_vocabulary" (
	"id"	INTEGER NOT NULL UNIQUE,
	"characters"	TEXT NOT NULL,
	"context_sentences"	TEXT NOT NULL,
	"meaning_mnemonic"	TEXT NOT NULL,
	"parts_of_speech"	TEXT NOT NULL,
	"pronunciation_auditos"	TEXT NOT NULL,
	FOREIGN KEY("id") REFERENCES "subjects"("id"),
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "kanji" (
	"id"	INTEGER NOT NULL UNIQUE,
	"characters"	TEXT NOT NULL,
	"reading_mnemonic"	TEXT NOT NULL,
	"reading_hint"	TEXT NOT NULL,
	"amalgamation_subject_ids"	TEXT NOT NULL,
	"component_subject_ids"	TEXT NOT NULL,
	"meaning_hint"	TEXT NOT NULL,
	"readings"	TEXT NOT NULL,
	"visually_similar_subject_ids"	TEXT NOT NULL,
	FOREIGN KEY("id") REFERENCES "subjects"("id"),
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "radicals" (
	"id"	INTEGER NOT NULL UNIQUE,
	"amalgamation_subject_ids"	TEXT NOT NULL,
	"auxilliary_meanings"	TEXT NOT NULL,
	"characters"	TEXT,
	"character_images"	TEXT NOT NULL,
	"created_at"	TEXT NOT NULL,
	"document_url"	TEXT NOT NULL,
	"hidden_at"	TEXT,
	"lesson_position"	INTEGER NOT NULL,
	"level"	INTEGER NOT NULL,
	"meanings"	TEXT NOT NULL,
	"meaning_mnemonic"	TEXT NOT NULL,
	"slug"	TEXT NOT NULL,
	"spaced_repetition_system_id"	INTEGER NOT NULL,
	FOREIGN KEY("id") REFERENCES "subjects"("id"),
	PRIMARY KEY("id")
);
