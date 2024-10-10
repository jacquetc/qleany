// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "database_test_persistence_export.h"
#include "repository/repository_provider.h"
#include <QObject>

namespace DatabaseTest::Persistence
{
class DATABASE_TEST_PERSISTENCE_EXPORT PersistenceRegistration : public QObject
{
    Q_OBJECT

  public:
    explicit PersistenceRegistration(QObject *parent);

    DatabaseTest::Persistence::Repository::RepositoryProvider *repositoryProvider();

  Q_SIGNALS:

  private:
    QScopedPointer<DatabaseTest::Persistence::Repository::RepositoryProvider> m_repositoryProvider;
};
} // namespace DatabaseTest::Persistence