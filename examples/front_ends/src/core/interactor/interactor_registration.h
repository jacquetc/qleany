// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

#pragma once

#include "front_ends_example_interactor_export.h"
#include <QObject>
#include <qleany/contracts/repository/interface_repository_provider.h>

namespace FrontEnds::Interactor
{

class FRONT_ENDS_EXAMPLE_INTERACTOR_EXPORT InteractorRegistration : public QObject
{
    Q_OBJECT
  public:
    explicit InteractorRegistration(QObject *parent,
                                    Qleany::Contracts::Repository::InterfaceRepositoryProvider *repositoryProvider);
    ~InteractorRegistration();

  private:
};

} // namespace FrontEnds::Interactor