// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "front_ends_example_persistence_export.h"
#include "repository/repository_provider.h"
#include <QObject>

namespace FrontEnds::Persistence
{
class FRONT_ENDS_EXAMPLE_PERSISTENCE_EXPORT PersistenceRegistration : public QObject
{
    Q_OBJECT

public:
    explicit PersistenceRegistration(QObject *parent);

    FrontEnds::Persistence::Repository::RepositoryProvider *repositoryProvider();

Q_SIGNALS:

private:
    QScopedPointer<FrontEnds::Persistence::Repository::RepositoryProvider> m_repositoryProvider;
};
} // namespace FrontEnds::Persistence