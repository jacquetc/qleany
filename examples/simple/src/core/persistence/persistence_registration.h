// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "simple_example_persistence_export.h"
#include <QObject>
#include <qleany/repository/repository_provider.h>

namespace Simple::Persistence
{
class SIMPLE_EXAMPLE_PERSISTENCE_EXPORT PersistenceRegistration : public QObject
{
    Q_OBJECT

  public:
    explicit PersistenceRegistration(QObject *parent);

    Qleany::Repository::RepositoryProvider *repositoryProvider();

  Q_SIGNALS:

  private:
    QScopedPointer<Qleany::Repository::RepositoryProvider> m_repositoryProvider;
};
} // namespace Simple::Persistence