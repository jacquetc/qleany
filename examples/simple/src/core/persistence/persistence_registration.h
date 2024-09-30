// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "repository/repository_provider.h"
#include "simple_example_persistence_export.h"
#include <QObject>

namespace Simple::Persistence
{
class SIMPLE_EXAMPLE_PERSISTENCE_EXPORT PersistenceRegistration : public QObject
{
    Q_OBJECT

  public:
    explicit PersistenceRegistration(QObject *parent);

    Simple::Persistence::Repository::RepositoryProvider *repositoryProvider();

  Q_SIGNALS:

  private:
    QScopedPointer<Simple::Persistence::Repository::RepositoryProvider> m_repositoryProvider;
};
} // namespace Simple::Persistence