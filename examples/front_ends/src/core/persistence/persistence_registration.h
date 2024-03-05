// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "front_ends_example_persistence_export.h"
#include <QObject>
#include <qleany/repository/repository_provider.h>

namespace FrontEnds::Persistence
{
class FRONT_ENDS_EXAMPLE_PERSISTENCE_EXPORT PersistenceRegistration : public QObject
{
    Q_OBJECT

  public:
    explicit PersistenceRegistration(QObject *parent);

    Qleany::Repository::RepositoryProvider *repositoryProvider();

  signals:

  private:
    QScopedPointer<Qleany::Repository::RepositoryProvider> m_repositoryProvider;
};
} // namespace FrontEnds::Persistence