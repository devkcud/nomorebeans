UPDATE profiles SET username = CONCAT('deleted:', id) WHERE deleted_at IS NOT NULL;
